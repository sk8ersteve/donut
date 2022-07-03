/* tslint:disable */
/* eslint-disable */
/**
*/
export class Screen {
  free(): void;
/**
* @param {number} w
* @param {number} h
* @returns {Screen}
*/
  static new(w: number, h: number): Screen;
/**
* @param {number} r
* @param {number} g
* @param {number} b
* @param {number} a
*/
  set_donut_color(r: number, g: number, b: number, a: number): void;
/**
*/
  draw_circle(): void;
/**
* @param {number} A
* @param {number} B
*/
  draw_donut(A: number, B: number): void;
/**
* @returns {number}
*/
  get_screen(): number;
/**
* @returns {number}
*/
  get_K1(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_screen_free: (a: number) => void;
  readonly screen_new: (a: number, b: number) => number;
  readonly screen_set_donut_color: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly screen_draw_circle: (a: number) => void;
  readonly screen_draw_donut: (a: number, b: number, c: number) => void;
  readonly screen_get_screen: (a: number) => number;
  readonly screen_get_K1: (a: number) => number;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
