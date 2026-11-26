import React, {useEffect, useRef, useState} from 'react';
import styles from './LiveExample.module.css';

export type Region = {x: number; y: number; width: number; height: number};
export type RegionSpec = {width: number; height: number};
export type Size = {width: number; height: number};

type Point = {x: number; y: number};
const MIN_BOX = 8;

/** Default template region centred horizontally at 40% vertical height. */
export function defaultRegion(size: Size, spec: RegionSpec): Region {
  const width = Math.min(Math.max(MIN_BOX, spec.width), size.width);
  const height = Math.min(Math.max(MIN_BOX, spec.height), size.height);
  const x = Math.max(0, Math.floor((size.width - width) / 2));
  const y = Math.max(0, Math.floor((size.height * 2) / 5) - Math.floor(height / 2));
  return clampRegion({x, y, width, height}, size);
}

/** Clamps a rectangular region to image pixel boundaries with minimum size. */
export function clampRegion(box: Region, size: Size): Region {
  const width = Math.min(Math.max(MIN_BOX, Math.round(box.width)), size.width);
  const height = Math.min(Math.max(MIN_BOX, Math.round(box.height)), size.height);
  const x = Math.min(Math.max(0, Math.round(box.x)), Math.max(0, size.width - width));
  const y = Math.min(Math.max(0, Math.round(box.y)), Math.max(0, size.height - height));
  return {x, y, width, height};
}

function toPoint(el: HTMLElement, clientX: number, clientY: number, size: Size): Point {
  const rect = el.getBoundingClientRect();
  const rx = rect.width > 0 ? (clientX - rect.left) / rect.width : 0;
  const ry = rect.height > 0 ? (clientY - rect.top) / rect.height : 0;
  return {
    x: Math.min(size.width, Math.max(0, Math.round(rx * size.width))),
    y: Math.min(size.height, Math.max(0, Math.round(ry * size.height))),
  };
}

function dragBox(start: Point, end: Point, size: Size): Region {
  return clampRegion(
    {
      x: Math.min(start.x, end.x),
      y: Math.min(start.y, end.y),
      width: Math.abs(end.x - start.x),
      height: Math.abs(end.y - start.y),
    },
    size,
  );
}

function recenterBox(pt: Point, cur: Region, size: Size): Region {
  return clampRegion(
    {x: pt.x - Math.floor(cur.width / 2), y: pt.y - Math.floor(cur.height / 2), width: cur.width, height: cur.height},
    size,
  );
}

function pctStyle(box: Region, size: Size): React.CSSProperties {
  return {
    left: `${(box.x / size.width) * 100}%`,
    top: `${(box.y / size.height) * 100}%`,
    width: `${(box.width / size.width) * 100}%`,
    height: `${(box.height / size.height) * 100}%`,
  };
}

type OverlayProps = {
  size: Size;
  region: Region;
  onChange: (next: Region) => void;
  onReset: () => void;
  children: React.ReactNode;
};

/** Interactive mouse drag / click overlay mapping DOM pointer events to image coordinates. */
export function RegionOverlay({size, region, onChange, onReset, children}: OverlayProps) {
  const surfaceRef = useRef<HTMLDivElement>(null);
  const [start, setStart] = useState<Point | null>(null);
  const [draft, setDraft] = useState<Region | null>(null);

  const down = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!surfaceRef.current) return;
    surfaceRef.current.setPointerCapture(e.pointerId);
    setStart(toPoint(surfaceRef.current, e.clientX, e.clientY, size));
    setDraft(null);
  };

  const move = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!start || !surfaceRef.current) return;
    const cur = toPoint(surfaceRef.current, e.clientX, e.clientY, size);
    if (Math.hypot(cur.x - start.x, cur.y - start.y) >= 4) {
      setDraft(dragBox(start, cur, size));
    }
  };

  const up = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!start || !surfaceRef.current) return;
    const end = toPoint(surfaceRef.current, e.clientX, e.clientY, size);
    const next =
      Math.hypot(end.x - start.x, end.y - start.y) >= 4
        ? dragBox(start, end, size)
        : recenterBox(end, region, size);
    setStart(null);
    setDraft(null);
    onChange(next);
  };

  return (
    <div className={styles.regionContainer}>
      <div
        ref={surfaceRef}
        className={styles.regionSurface}
        onPointerDown={down}
        onPointerMove={move}
        onPointerUp={up}>
        {children}
        <div className={styles.regionBox} style={pctStyle(region, size)} />
        {draft && <div className={styles.regionDraft} style={pctStyle(draft, size)} />}
      </div>
      <div className={styles.regionHint}>
        <span>
          Template: ({region.x}, {region.y}) · {region.width}×{region.height} — drag or click Input
        </span>
        <button type="button" onClick={onReset}>
          Reset region
        </button>
      </div>
    </div>
  );
}

/** Appends `(x, y, width, height)` when a template region is active. */
export function withRegion(base: number[], region?: Region): number[] {
  return region ? [...base, region.x, region.y, region.width, region.height] : base;
}

/** Tracks the active image region, resetting to default when the input photo changes. */
export function useRegion(size?: Size, spec?: RegionSpec) {
  const [region, setRegion] = useState<Region>();
  useEffect(() => {
    setRegion(size && spec ? defaultRegion(size, spec) : undefined);
  }, [size, spec?.width, spec?.height]);
  const reset = () => {
    if (size && spec) setRegion(defaultRegion(size, spec));
  };
  return {region, setRegion, reset};
}

/** Renders the Input figure with optional interactive region selection overlaid. */
export function InputFigure({
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
