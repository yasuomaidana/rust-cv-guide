import React, {useCallback, useEffect, useMemo, useRef, useState} from 'react';
import BrowserOnly from '@docusaurus/BrowserOnly';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {defaultRegion, Region, RegionOverlay, RegionSpec, Size} from './RegionOverlay';
import styles from './LiveExample.module.css';

/** A control above the pictures. With `options` it is a drop-down whose value is the option's index. */
export type Param = {
  name: string;
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  options?: string[];
};

type Props = {
  /** Exported function of the `cvg-live` crate, called as `fn(pixels, [secondPixels,] width, height, ...params)`. */
  fn: string;
  /** Photo in `snippets/assets`, served from `/samples/`. */
  sample: string;
  /** Second photo for examples that combine two images; it is resized to the first one's size. */
  second?: string;
  params?: Param[];
  /** Optional independent controls for `alt.fn`; when omitted, `params` drives both. */
  altParams?: Param[];
  /** Optional interactive rectangular region selector overlaid on the Input image, appended as (x, y, width, height). */
  selectableRegion?: RegionSpec;
  /** Caption of the output of `fn`; defaults to "Output". */
  label?: string;
  /** A second implementation of the same job (another crate), run on the same pixels and controls and drawn beside the first. */
  alt?: {fn: string; label: string};
  /** An exported function that returns text, called with the same arguments and shown under the pictures (numbers such as PSNR). */
  readout?: {fn: string; label: string};
  /** Optional second readout function for comparing metrics and timings side by side. */
  altReadout?: {fn: string; label: string};
};

const MAX_SIDE = 640;
type Wasm = Record<string, (...args: unknown[]) => Uint8Array & string>;
let wasmLoading: Promise<Wasm> | undefined;

/** `version` changes with every build, so a rebuilt bundle is never served from the browser cache. */
function loadWasm(base: string, version: string): Promise<Wasm> {
  wasmLoading ??= import(/* webpackIgnore: true */ `${base}wasm/cvg_live.js?v=${version}`).then(async (module) => {
    // The glue resolves the .wasm without our query, so it is given explicitly.
    await module.default({module_or_path: `${base}wasm/cvg_live_bg.wasm?v=${version}`});
    return module as Wasm;
  });
  return wasmLoading;
}

function loadImage(url: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => resolve(image);
    image.onerror = () => reject(new Error(`cannot load ${url}`));
    image.src = url;
  });
}

/** Draws the image scaled to fit MAX_SIDE (or to the given size) and returns its pixels. */
function pixelsOf(image: HTMLImageElement, size?: {width: number; height: number}): ImageData {
  const scale = Math.min(1, MAX_SIDE / Math.max(image.naturalWidth, image.naturalHeight));
  const width = size?.width ?? Math.max(1, Math.round(image.naturalWidth * scale));
  const height = size?.height ?? Math.max(1, Math.round(image.naturalHeight * scale));
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const context = canvas.getContext('2d')!;
  context.drawImage(image, 0, 0, width, height);
  return context.getImageData(0, 0, width, height);
}

function show(canvas: HTMLCanvasElement | null, data: ImageData) {
  if (!canvas) return;
  canvas.width = data.width;
  canvas.height = data.height;
  canvas.getContext('2d')!.putImageData(data, 0, 0);
}

type Pictures = {first: ImageData; other?: ImageData};

/** The sample photo(s), reloadable; `setPictures` swaps in the reader's own image. */
function useSamples(base: string, sample: string, second?: string) {
  const [pictures, setPictures] = useState<Pictures>();
  const [error, setError] = useState<string>();
  const reload = useCallback(async () => {
    try {
      const first = pixelsOf(await loadImage(`${base}samples/${sample}`));
      const other = second ? pixelsOf(await loadImage(`${base}samples/${second}`), first) : undefined;
      setPictures({first, other});
      setError(undefined);
    } catch (failure) {
      setError(String(failure));
    }
  }, [base, sample, second]);
  useEffect(() => {
    void reload();
  }, [reload]);
  return {pictures, setPictures, error, reload};
}

const bytes = (data: ImageData) => new Uint8Array(data.data.buffer);

/** How long one solution took, or why it could not run. */
type Outcome = {ms: number} | {error: string};
type Outcomes = {main?: Outcome; alt?: Outcome};
type ReadoutResult = {text: string; ms: number} | {error: string};
type Readouts = {main?: ReadoutResult; alt?: ReadoutResult};

const describe = (failure: unknown) => String(failure instanceof Error ? failure.message : failure);

/** What a wasm function is called with: the pictures' bytes, their size and the controls' values. */
type Call = {wasm: Wasm; images: Uint8Array[]; size: {width: number; height: number}; values: number[]};

const invoke = ({wasm, images, size, values}: Call, name: string) => wasm[name](...images, size.width, size.height, ...values);

/** Draws the result of one exported function and times it; a missing export or a failure is reported, not thrown. */
function runOne(call: Call, name: string, canvas: HTMLCanvasElement | null): Outcome {
  if (typeof call.wasm[name] !== 'function') {
    return {error: `cvg-live has no export "${name}": rebuild the wasm (bun run wasm) and hard-reload`};
  }
  try {
    const started = performance.now();
    show(canvas, new ImageData(new Uint8ClampedArray(invoke(call, name)), call.size.width, call.size.height));
    return {ms: Math.round(performance.now() - started)};
  } catch (failure) {
    return {error: `"${name}" failed: ${describe(failure)}`};
  }
}

/** Runs a text readout export and times it independently. */
function readoutOf(call: Call, name: string): ReadoutResult {
  if (typeof call.wasm[name] !== 'function') {
    return {error: `no export "${name}"`};
  }
  try {
    const started = performance.now();
    const text = String(invoke(call, name));
    return {text, ms: Math.round(performance.now() - started)};
  } catch (failure) {
    return {error: `"${name}" failed: ${describe(failure)}`};
  }
}

function evaluateRun(
  wasm: Wasm,
  pictures: Pictures,
  fn: string,
  extra: {altFn?: string; readoutFn?: string; altReadoutFn?: string},
  canvases: {input: HTMLCanvasElement | null; output: HTMLCanvasElement | null; altOutput: HTMLCanvasElement | null},
  mainValues: number[],
  altValues: number[],
) {
  const {first, other} = pictures;
  const images = other ? [bytes(first), bytes(other)] : [bytes(first)];
  const mainCall: Call = {wasm, images, size: first, values: mainValues};
  const altCall: Call = {wasm, images, size: first, values: altValues};
  show(canvases.input, first);
  return {
    outcomes: {
      main: runOne(mainCall, fn, canvases.output),
      alt: extra.altFn ? runOne(altCall, extra.altFn, canvases.altOutput) : undefined,
    },
    readouts: {
      main: extra.readoutFn ? readoutOf(mainCall, extra.readoutFn) : undefined,
      alt: extra.altReadoutFn ? readoutOf(altCall, extra.altReadoutFn) : undefined,
    },
  };
}

/** Runs the wasm function whenever the pictures or the controls change and draws both canvases, timing each one on its own. */
function useRun(
  base: string,
  version: string,
  fn: string,
  extra: {altFn?: string; readoutFn?: string; altReadoutFn?: string},
  pictures: Pictures | undefined,
  mainValues: number[],
  altValues: number[],
) {
  const input = useRef<HTMLCanvasElement>(null);
  const output = useRef<HTMLCanvasElement>(null);
  const altOutput = useRef<HTMLCanvasElement>(null);
  const [status, setStatus] = useState('Loading…');
  const [outcomes, setOutcomes] = useState<Outcomes>({});
  const [readouts, setReadouts] = useState<Readouts>({});
  useEffect(() => {
    if (!pictures) return;
    let cancelled = false;
    loadWasm(base, version)
      .then((wasm) => {
        if (cancelled) return;
        const next = evaluateRun(
          wasm,
          pictures,
          fn,
          extra,
          {input: input.current, output: output.current, altOutput: altOutput.current},
          mainValues,
          altValues,
        );
        setOutcomes(next.outcomes);
        setReadouts(next.readouts);
        setStatus('Computed in your browser');
      })
      .catch((failure) => setStatus(`Could not run the example: ${describe(failure)}`));
    return () => {
      cancelled = true;
    };
  }, [base, version, fn, extra.altFn, extra.readoutFn, extra.altReadoutFn, pictures, mainValues, altValues]);
  return {input, output, altOutput, status, outcomes, readouts};
}

/** The caption of one output with its own time (or the reason it did not run). */
function Caption({label, outcome}: {label: string; outcome?: Outcome}) {
  return (
    <figcaption>
      {label}
      {outcome && 'ms' in outcome && <span className={styles.timing}> · {outcome.ms} ms</span>}
      {outcome && 'error' in outcome && <span className={styles.failure}>{outcome.error}</span>}
    </figcaption>
  );
}

function Control({param, value, onChange}: {param: Param; value: number; onChange: (value: number) => void}) {
  const choose = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => onChange(Number(e.target.value));
  const field = param.options ? (
    <select value={value} onChange={choose}>
      {param.options.map((option, i) => (
        <option key={option} value={i}>
          {option}
        </option>
      ))}
    </select>
  ) : (
    <input type="range" min={param.min} max={param.max} step={param.step} value={value} onChange={choose} />
  );
  return (
    <label className={styles.control}>
      <span>
        {param.label}
        {!param.options && <strong> {value}</strong>}
      </span>
      {field}
    </label>
  );
}

function ControlRow({params, values, onChange}: {params: Param[]; values: number[]; onChange: (i: number, v: number) => void}) {
  return (
    <div className={styles.controls}>
      {params.map((param, index) => (
        <Control key={param.name} param={param} value={values[index]} onChange={(v) => onChange(index, v)} />
      ))}
    </div>
  );
}

function ReadoutBody({result}: {result?: ReadoutResult}) {
  if (!result) return <>…</>;
  if ('error' in result) return <span className={styles.failure}>{result.error}</span>;
  return <>{result.text}</>;
}

function ReadoutCard({label, result}: {label: string; result?: ReadoutResult}) {
  return (
    <div className={styles.readoutCard}>
      <div className={styles.readoutHeader}>
        <strong>{label}</strong>
        {result && 'ms' in result && <span className={styles.timing}> · {result.ms} ms</span>}
      </div>
      <div className={styles.readoutValue}>
        <ReadoutBody result={result} />
      </div>
    </div>
  );
}

function ReadoutCards({
  readoutSpec,
  altReadoutSpec,
  readouts,
}: {
  readoutSpec?: {fn: string; label: string};
  altReadoutSpec?: {fn: string; label: string};
  readouts: Readouts;
}) {
  if (!readoutSpec) return null;
  if (!altReadoutSpec) {
    const r = readouts.main;
    return (
      <p className={styles.readout}>
        <strong>{readoutSpec.label}:</strong> <ReadoutBody result={r} />
        {r && 'ms' in r && <span className={styles.timing}> · {r.ms} ms</span>}
      </p>
    );
  }
  return (
    <div className={styles.readoutGrid}>
      <ReadoutCard label={readoutSpec.label} result={readouts.main} />
      <ReadoutCard label={altReadoutSpec.label} result={readouts.alt} />
    </div>
  );
}

function Footer({status, second, onFile, onReset}: {status: string; second: boolean; onFile: (f?: File) => void; onReset: () => void}) {
  return (
    <div className={styles.footer}>
      <label className={styles.upload}>
        Use your own image
        <input type="file" accept="image/*" onChange={(e) => onFile(e.target.files?.[0])} />
      </label>
      {second ? <span>(the second picture stays the sample)</span> : null}
      <button type="button" onClick={onReset}>
        Back to the sample
      </button>
      <span className={styles.status}>{status}. Your image never leaves this page.</span>
    </div>
  );
}

function InputFigure({
  canvasRef,
  size,
  region,
  onChange,
  onReset,
}: {
  canvasRef: React.RefObject<HTMLCanvasElement | null>;
  size?: Size;
  region?: Region;
  onChange: (next: Region) => void;
  onReset: () => void;
}) {
  const canvas = <canvas ref={canvasRef} />;
  return (
    <figure>
      {size && region ? (
        <RegionOverlay size={size} region={region} onChange={onChange} onReset={onReset}>
          {canvas}
        </RegionOverlay>
      ) : (
        canvas
      )}
      <figcaption>Input</figcaption>
    </figure>
  );
}

function withRegion(base: number[], region?: Region): number[] {
  return region ? [...base, region.x, region.y, region.width, region.height] : base;
}

function useRegion(pictures?: Pictures, spec?: RegionSpec) {
  const [region, setRegion] = useState<Region>();
  useEffect(() => {
    setRegion(pictures && spec ? defaultRegion(pictures.first, spec) : undefined);
  }, [pictures, spec?.width, spec?.height]);
  const reset = () => {
    if (pictures && spec) setRegion(defaultRegion(pictures.first, spec));
  };
  return {region, setRegion, reset};
}

type ControlsProps = {
  params: Param[];
  altParams?: Param[];
  values: number[];
  altValues: number[];
  synced: boolean;
  label: string;
  altLabel?: string;
  onMain: (index: number, value: number) => void;
  onAlt: (index: number, value: number) => void;
  onToggleSync: (next: boolean) => void;
};

function ControlsPanel(p: ControlsProps) {
  const dual = Boolean(p.altParams && p.altLabel);
  return (
    <>
      {dual && (
        <div className={styles.syncBar}>
          <label>
            <input type="checkbox" checked={p.synced} onChange={(e) => p.onToggleSync(e.target.checked)} /> Sync
            parameters between {p.label} and {p.altLabel}
          </label>
        </div>
      )}
      {dual && !p.synced && p.altParams ? (
        <div className={styles.splitControls}>
          <div className={styles.controlGroup}>
            <div className={styles.groupTitle}>{p.label}</div>
            <ControlRow params={p.params} values={p.values} onChange={p.onMain} />
          </div>
          <div className={styles.controlGroup}>
            <div className={styles.groupTitle}>{p.altLabel}</div>
            <ControlRow params={p.altParams} values={p.altValues} onChange={p.onAlt} />
          </div>
        </div>
      ) : (
        <ControlRow params={p.params} values={p.values} onChange={p.onMain} />
      )}
    </>
  );
}

function Live({
  fn,
  sample,
  second,
  params = [],
  altParams,
  selectableRegion,
  label = 'Output',
  alt,
  readout: readoutSpec,
  altReadout,
}: Props) {
  const base = useBaseUrl('/');
  const version = String(useDocusaurusContext().siteConfig.customFields?.buildTime ?? '');
  const [values, setValues] = useState(params.map((p) => p.value));
  const [altValues, setAltValues] = useState((altParams ?? params).map((p) => p.value));
  const [synced, setSynced] = useState(Boolean(altParams && params.length === altParams.length));
  const {pictures, setPictures, error, reload} = useSamples(base, sample, second);
  const {region, setRegion, reset: resetRegion} = useRegion(pictures, selectableRegion);

  const effectiveAltValues = altParams ? (synced ? values : altValues) : values;
  const runMainValues = useMemo(() => withRegion(values, region), [values, region]);
  const runAltValues = useMemo(() => withRegion(effectiveAltValues, region), [effectiveAltValues, region]);
  const {input, output, altOutput, status, outcomes, readouts} = useRun(
    base,
    version,
    fn,
    {altFn: alt?.fn, readoutFn: readoutSpec?.fn, altReadoutFn: altReadout?.fn},
    pictures,
    runMainValues,
    runAltValues,
  );

  const upload = async (file?: File) => {
    if (!file) return;
    const url = URL.createObjectURL(file);
    try {
      setPictures({first: pixelsOf(await loadImage(url))});
    } finally {
      URL.revokeObjectURL(url);
    }
  };
  const changeMain = (index: number, value: number) =>
    setValues((current) => current.map((old, i) => (i === index ? value : old)));
  const changeAlt = (index: number, value: number) =>
    setAltValues((current) => current.map((old, i) => (i === index ? value : old)));
  const toggleSync = (next: boolean) => {
    setSynced(next);
    if (next) setAltValues(values);
  };

  return (
    <div className={styles.live}>
      <ControlsPanel
        params={params}
        altParams={altParams}
        values={values}
        altValues={altValues}
        synced={synced}
        label={label}
        altLabel={alt?.label}
        onMain={changeMain}
        onAlt={changeAlt}
        onToggleSync={toggleSync}
      />
      <div className={styles.pictures}>
        <InputFigure canvasRef={input} size={pictures?.first} region={region} onChange={setRegion} onReset={resetRegion} />
        <figure>
          <canvas ref={output} />
          <Caption label={label} outcome={outcomes.main} />
        </figure>
        {alt && (
          <figure>
            <canvas ref={altOutput} />
            <Caption label={alt.label} outcome={outcomes.alt} />
          </figure>
        )}
      </div>
      <ReadoutCards readoutSpec={readoutSpec} altReadoutSpec={altReadout} readouts={readouts} />
      <Footer status={error ?? status} second={Boolean(second)} onFile={(f) => void upload(f)} onReset={() => void reload()} />
    </div>
  );
}

export default function LiveExample(props: Props): React.ReactElement {
  return <BrowserOnly fallback={<p>Loading the live example…</p>}>{() => <Live {...props} />}</BrowserOnly>;
}
