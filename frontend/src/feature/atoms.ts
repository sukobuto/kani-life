import {atom} from "jotai";


export const gameFieldSizeAtom = atom(30);
export const foodSizeMaxAtom = atom(3);

export type PaintedCell = {
    x: number
    y: number
    color: string
}
export const paintedCellsAtom = atom<PaintedCell[]>([]);


export type Food = {
    id: string
    position: {
        x: number
        y: number
    }
    size: number
}
export const foodsAtom = atom<Food[]>([]);


export type Crab = {
    name: string
    hue: number
    point: number
    direction: "N" | "E" | "S" | "W"
    position: {
        x: number
        y: number
    }
}
export const crabsAtom = atom<Crab[]>([
    // {name: "test001", hue: 370, point: 0, direction: "N", position: {x: 10, y: 10}},
    // {name: "test002", hue: 280, point: 0, direction: "N", position: {x: 15, y: 3}},
    // {name: "test003", hue: 190, point: 0, direction: "S", position: {x: 5, y: 15}},
]);
