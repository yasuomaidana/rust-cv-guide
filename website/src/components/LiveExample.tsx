import React, {useCallback, useEffect, useRef, useState} from 'react';
import BrowserOnly from '@docusaurus/BrowserOnly';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
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
  /** Caption of the output of `fn`; defaults to "Output". */
  label?: string;
  /** A second implementation of the same job (another crate), run on the same pixels and controls and drawn beside the first. */
  alt?: {fn: string; label: string};
  /** An exported function that returns text, called with the same arguments and shown under the pictures (numbers such as PSNR). */
  readout?: {fn: string; label: string};
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

/** The text an exported function returns, or why there is none. */
function readoutOf(call: Call, name: string): string {
  return typeof call.wasm[name] === 'function' ? String(invoke(call, name)) : `no export "${name}"`;
}

/** Runs the wasm function whenever the pictures or the controls change and draws both canvases, timing each one on its own. */
function useRun(
  base: string,
  version: string,
  fn: string,
  extra: {altFn?: string; readoutFn?: string},
  pictures: Pictures | undefined,
  values: number[],
) {
  const {altFn, readoutFn} = extra;
  const input = useRef<HTMLCanvasElement>(null);
  const output = useRef<HTMLCanvasElement>(null);
  const altOutput = useRef<HTMLCanvasElement>(null);
  const [status, setStatus] = useState('Loading…');
  const [outcomes, setOutcomes] = useState<Outcomes>({});
  const [readout, setReadout] = useState('');
  useEffect(() => {
    if (!pictures) return;
    let cancelled = false;
    const {first, other} = pictures;
    loadWasm(base, version)
      .then((wasm) => {
        if (cancelled) return;
        const call: Call = {wasm, images: other ? [bytes(first), bytes(other)] : [bytes(first)], size: first, values};
        show(input.current, first);
        setOutcomes({main: runOne(call, fn, output.current), alt: altFn ? runOne(call, altFn, altOutput.current) : undefined});
        if (readoutFn) setReadout(readoutOf(call, readoutFn));
        setStatus('Computed in your browser');
      })
      .catch((failure) => setStatus(`Could not run the example: ${describe(failure)}`));
    return () => {
      cancelled = true;
    };
  }, [base, version, fn, altFn, readoutFn, pictures, values]);
  return {input, output, altOutput, status, outcomes, readout};
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

function Live({fn, sample, second, params = [], label = 'Output', alt, readout: readoutSpec}: Props) {
  const base = useBaseUrl('/');
  const version = String(useDocusaurusContext().siteConfig.customFields?.buildTime ?? '');
  const [values, setValues] = useState(params.map((p) => p.value));
  const {pictures, setPictures, error, reload} = useSamples(base, sample, second);
  const {input, output, altOutput, status, outcomes, readout} = useRun(
    base,
    version,
    fn,
    {altFn: alt?.fn, readoutFn: readoutSpec?.fn},
    pictures,
    values,
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
  const change = (index: number, value: number) =>
    setValues((current) => current.map((old, i) => (i === index ? value : old)));

  return (
    <div className={styles.live}>
      <div className={styles.controls}>
        {params.map((param, index) => (
          <Control key={param.name} param={param} value={values[index]} onChange={(value) => change(index, value)} />
        ))}
      </div>
      <div className={styles.pictures}>
        <figure>
          <canvas ref={input} />
          <figcaption>Input</figcaption>
        </figure>
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
      {readoutSpec && (
        <p className={styles.readout}>
          <strong>{readoutSpec.label}:</strong> {readout}
        </p>
      )}
      <Footer status={error ?? status} second={Boolean(second)} onFile={(f) => void upload(f)} onReset={() => void reload()} />
    </div>
  );
}

export default function LiveExample(props: Props): React.ReactElement {
  return <BrowserOnly fallback={<p>Loading the live example…</p>}>{() => <Live {...props} />}</BrowserOnly>;
}
