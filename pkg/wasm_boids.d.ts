/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export class Parameters {
  free(): void;
}
/**
*/
export class Swarm {
  free(): void;
/**
*/
  tick(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Swarm}
*/
  static new(width: number, height: number): Swarm;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  number_boids(): number;
/**
* @returns {number}
*/
  x(): number;
/**
* @returns {number}
*/
  y(): number;
/**
* @returns {number}
*/
  dx(): number;
/**
* @returns {number}
*/
  dy(): number;
/**
* @param {number} width
*/
  set_width(width: number): void;
/**
* @param {number} height
*/
  set_height(height: number): void;
/**
* @param {number} border
*/
  set_border(border: number): void;
/**
* @param {number} separation_min_distance
*/
  set_separation_min_distance(separation_min_distance: number): void;
/**
* @param {number} cohesion
*/
  set_cohesion(cohesion: number): void;
/**
* @param {number} separation
*/
  set_separation(separation: number): void;
/**
* @param {number} alignment
*/
  set_alignment(alignment: number): void;
/**
* @param {number} vision_range
*/
  set_visual_range(vision_range: number): void;
/**
* @param {number} speed_limit
*/
  set_speed_limit(speed_limit: number): void;
/**
* @param {boolean} checked
*/
  set_mouse_interactivity(checked: boolean): void;
/**
* @param {boolean} checked
*/
  set_follow_fear(checked: boolean): void;
/**
* @param {number} x
* @param {number} y
*/
  set_mouse_position(x: number, y: number): void;
}
