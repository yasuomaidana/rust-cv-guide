import React, {useCallback, useEffect, useRef, useState} from 'react';
import BrowserOnly from '@docusaurus/BrowserOnly';
import useBaseUrl from '@docusaurus/useBaseUrl';
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
};

const MAX_SIDE = 640;
type Wasm = Record<string, (...args: unknown[]) => Uint8Array>;
let wasmLoading: Promise<Wasm> | undefined;

function loadWasm(base: string): Promise<Wasm> {
  wasmLoading ??= import(/* webpackIgnore: true */ `${base}wasm/cvg_live.js`).then(async (module) => {
    await module.default();
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

/** Runs the wasm function whenever the pictures or the controls change and draws both canvases. */
function useRun(base: string, fn: string, pictures: Pictures | undefined, values: number[]) {
  const input = useRef<HTMLCanvasElement>(null);
  const output = useRef<HTMLCanvasElement>(null);
  const [status, setStatus] = useState('Loading…');
  useEffect(() => {
    if (!pictures) return;
    let cancelled = false;
    const {first, other} = pictures;
    loadWasm(base)
      .then((wasm) => {
        if (cancelled) return;
        const started = performance.now();
        const images = other ? [bytes(first), bytes(other)] : [bytes(first)];
        const result = wasm[fn](...images, first.width, first.height, ...values);
        show(input.current, first);
        show(output.current, new ImageData(new Uint8ClampedArray(result), first.width, first.height));
        setStatus(`Computed in your browser in ${Math.round(performance.now() - started)} ms`);
      })
      .catch((failure) => setStatus(`Could not run the example: ${failure}`));
    return () => {
      cancelled = true;
    };
  }, [base, fn, pictures, values]);
  return {input, output, status};
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

function Live({fn, sample, second, params = []}: Props) {
  const base = useBaseUrl('/');
  const [values, setValues] = useState(params.map((p) => p.value));
  const {pictures, setPictures, error, reload} = useSamples(base, sample, second);
  const {input, output, status} = useRun(base, fn, pictures, values);

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
          <figcaption>Output</figcaption>
        </figure>
      </div>
      <Footer status={error ?? status} second={Boolean(second)} onFile={(f) => void upload(f)} onReset={() => void reload()} />
    </div>
  );
}

export default function LiveExample(props: Props): React.ReactElement {
  return <BrowserOnly fallback={<p>Loading the live example…</p>}>{() => <Live {...props} />}</BrowserOnly>;
}
