import styled from "styled-components";
import {range} from "../feature/helper.ts";
import {useAtomValue} from "jotai";
import {
    type Crab,
    crabsAtom,
    decoratedCellsAtom,
    foodsAtom,
    foodSizeMaxAtom,
    gameFieldSizeAtom,
    paintedCellsAtom
} from "../feature/atoms.ts";
import {useDecoration} from "../feature/use-decoration.ts";
import {useWebSocket} from "../feature/use-websocket.ts";

type GridLineInfo = {
    lineNo: number
    cells: GridCellInfo[]
}

type GridCellInfo = {
    cellNo: number
}


function GameField() {
    useDecoration()
    useWebSocket()
    const gameFieldSize = useAtomValue(gameFieldSizeAtom)
    const lines: GridLineInfo[] = range(0, gameFieldSize - 1).map((lineNo) => {
        return {
            lineNo,
            cells:
                range(0, gameFieldSize - 1).map((cellNo) => {
                    return {
                        cellNo,
                    }
                })
        }
    })
    const crabs = useAtomValue(crabsAtom)
    return (
        <GameFieldContainer>
            {lines.map((line) => (
                <GridLine key={line.lineNo} $gameFieldSize={gameFieldSize}>
                    {line.cells.map((cell) => (
                        <GridCell key={`${line.lineNo}-${cell.cellNo}`} lineNo={line.lineNo} cellNo={cell.cellNo}/>
                    ))}
                </GridLine>
            ))}
            <CrabLayer>
                {crabs.map(crab => (
                    <Crab key={crab.name} info={crab}/>
                ))}
            </CrabLayer>
        </GameFieldContainer>
    )
}

const CrabLayer = styled.div`
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
`;

const GameFieldContainer = styled.div`
    position: relative;
    width: 80vh;
    height: 80vh;
    border: solid 1px rgba(116, 123, 255, 0.25);
`;

/**
 * セルを横に並べた1行のコンポーネント
 */
const GridLine = styled.div<{ $gameFieldSize: number; }>`
    display: flex;
    width: 80vh;
    height: ${props => 80 / props.$gameFieldSize}vh;
`;

type GridCellProps = {
    lineNo: number
    cellNo: number
}

function GridCell({lineNo, cellNo}: GridCellProps) {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom)
    const decoratedCells = useAtomValue(decoratedCellsAtom)
    const paintedCells = useAtomValue(paintedCellsAtom)
    const key = `${cellNo},${lineNo}`
    const color = paintedCells[key] ?? decoratedCells[key]
    const food = useAtomValue(foodsAtom).find((f) => f.position.y == lineNo && f.position.x == cellNo)
    return (
        <GridCellInner $gameFieldSize={gameFieldSize} style={{backgroundColor: color}}>
            {food && (
                <Food size={food.size}/>
            )}
        </GridCellInner>
    )
}

/**
 * 1つのセル
 */
const GridCellInner = styled.div<{ $paintColor?: string; $gameFieldSize: number; }>`
    box-sizing: border-box;
    width: ${props => 80 / props.$gameFieldSize}vh;
    height: ${props => 80 / props.$gameFieldSize}vh;
    border: solid 1px rgba(116, 123, 255, 0.15);
    background-color: ${props => props.$paintColor || "transparent"}
`;

type FoodProps = {
    size: number
}

function Food({size}: FoodProps) {
    const foodSizeMax = useAtomValue(foodSizeMaxAtom)
    // r を 5 から 15 の範囲にする
    const r = ((size - 1) / (foodSizeMax - 1)) * 6 + 6
    return (
        <FoodWrap>
            <svg viewBox="0 0 36 36">
                <circle cx="18" cy="18" r={r} fill="#FD8075"/>
                <circle cx="15" cy="15" r={r - 5} fill="#FFAAAA"/>
            </svg>
        </FoodWrap>
    )
}

const FoodWrap = styled.div`
    width: 100%;
    height: 100%;
`;

type CrabProps = {
    info: Crab
}

function Crab({info}: CrabProps) {
    const gameFieldSize = useAtomValue(gameFieldSizeAtom)
    const darker = `hsl(${info.hue}deg 95% 32%)`
    const baseColor = `hsl(${info.hue}deg 77% 42%)`
    const highlight = `hsl(${info.hue}deg 72% 52%)`
    return (
        <CrabBase $gameFieldSize={gameFieldSize}
                  style={crabPosition(info.position.x, info.position.y, gameFieldSize)}>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 36 36"
                style={{transform: `rotate(${direction2rotate(info.direction)}deg)`}}
            >
                <path fill={darker}
                      d="M6.96 20.637c.068.639-.543 1.228-1.368 1.315-.824.089-1.547-.357-1.615-.995-.068-.639.544-1.227 1.368-1.314.824-.089 1.547.356 1.615.994zm2.087 2.717c.125.818-1.756 2.544-2.576 2.669-.819.125-1.584-.438-1.708-1.257-.125-.818.58-1.14 1.398-1.265.819-.124 2.761-.965 2.886-.147zm1.783 2.104c.173.81-1.628 3.927-2.438 4.1-.811.173-1.645.146-1.817-.665-.173-.81.306-1.688 1.116-1.861.81-.174 2.966-2.384 3.139-1.574zm3.853.858c.165.811-1.338 4.354-2.15 4.519-.812.165-1.439.451-1.604-.36-.165-.812.261-1.975 1.006-2.58.644-.523 2.584-2.39 2.748-1.579z"/>
                <path fill={darker}
                      d="M3.925 18.869c.607.715 1.18 1.23.464 1.835-.716.606-1.747.162-2.354-.554-.605-.715-2.239-3.42-1.524-4.025.717-.606 2.809 2.029 3.414 2.744zm.33 4.88c.892.295 1.857.801 1.563 1.691-.294.891-1.328.991-2.219.698-.889-.295-3.772-1.691-3.478-2.581.295-.89 3.244-.102 4.134.192zm1.214 4.532c.905-.253 1.907-.283 2.159.619.252.903-.282 1.535-1.186 1.787-.902.251-4.342.727-4.594-.176-.251-.905 2.718-1.98 3.621-2.23zm4.348 3.188c.084-.937.644-1.669 1.577-1.585.934.085 1.258 1.025 1.173 1.96-.085.934.147 3.562-1.715 4.016-.912.221-1.121-3.46-1.035-4.391zM29.04 20.637c-.068.639.543 1.228 1.367 1.315.824.089 1.547-.357 1.615-.995.068-.639-.544-1.227-1.367-1.314-.824-.089-1.547.356-1.615.994zm-2.087 2.717c-.125.818 1.757 2.544 2.575 2.669.819.125 1.584-.438 1.709-1.257s-.58-1.14-1.398-1.265c-.819-.124-2.761-.965-2.886-.147zm-1.783 2.104c-.173.81 1.628 3.927 2.438 4.1.81.173 1.644.146 1.816-.665.174-.81-.305-1.688-1.115-1.861-.81-.174-2.966-2.384-3.139-1.574zm-3.853.858c-.166.811 1.338 4.354 2.149 4.519.812.165 1.438.451 1.604-.36.164-.812-.262-1.975-1.007-2.58-.642-.523-2.582-2.39-2.746-1.579z"/>
                <path fill={darker}
                      d="M32.075 18.869c-.607.715-1.18 1.23-.465 1.835.716.606 1.747.162 2.354-.554.605-.715 2.239-3.42 1.523-4.025-.715-.606-2.807 2.029-3.412 2.744zm-.33 4.88c-.892.295-1.857.801-1.562 1.691.293.891 1.328.991 2.219.698.889-.295 3.771-1.691 3.477-2.581-.294-.89-3.244-.102-4.134.192zm-1.215 4.532c-.904-.253-1.906-.283-2.158.619-.252.903.282 1.535 1.185 1.787.902.251 4.343.727 4.594-.177.252-.904-2.717-1.979-3.621-2.229zm-4.347 3.188c-.084-.937-.645-1.669-1.577-1.585-.935.085-1.258 1.025-1.173 1.96.084.934-.147 3.562 1.715 4.016.912.221 1.121-3.46 1.035-4.391zM3.148 13.878c-.383-.856.001-1.86.857-2.242.856-.383 1.86.002 2.243.858.381.855 2.651 5.612 1.795 5.994-.855.382-4.513-3.755-4.895-4.61z"/>
                <path fill={darker}
                      d="M3.994 12.034c1.221 2.956 8.341-3.341 8.803-6.281.462-2.939-.308-4.201-.694-4.5-.386-.299.144 1.435-1.187 3.306-1.053 1.482-7.766 5.434-6.922 7.475zm28.858 1.844c.384-.856-.001-1.86-.857-2.242-.857-.383-1.861.002-2.244.858-.381.855-2.65 5.612-1.794 5.994.855.382 4.513-3.755 4.895-4.61z"/>
                <path fill={darker}
                      d="M32.007 12.034c-1.222 2.956-8.341-3.341-8.804-6.281-.461-2.939.309-4.201.694-4.5.386-.299-.144 1.435 1.187 3.306 1.054 1.482 7.766 5.434 6.923 7.475z"/>
                <path fill={baseColor}
                      d="M6 22c0-2 2-10 12-10s12 8 12 10c-5 3-5.373 7-12 7s-6-4-12-7zm-1.677-9.777c-3.153.543-.358-8.141 1.883-10.099C8.446.166 10.863.207 11.321.374s-1.174 2.595-1.75 4.178c-.293.803-3.072 7.296-5.248 7.671zm27.354 0c3.152.543.358-8.141-1.882-10.099C27.555.166 25.139.207 24.68.374c-.459.167 1.174 2.595 1.75 4.178.293.803 3.071 7.296 5.247 7.671z"/>
                <path fill={darker}
                      d="M17.032 12.136c.335 1.339-.045 1.588-.849 1.789-.804.201-1.727.278-2.061-1.061-.335-1.339.045-2.588.849-2.789.803-.201 1.726.721 2.061 2.061zm4.846.728c-.335 1.34-1.258 1.262-2.061 1.061-.804-.201-1.184-.45-.849-1.789.335-1.34 1.258-2.262 2.062-2.061.803.2 1.183 1.449.848 2.789z"/>
                <circle fill="#292F33" cx="14.5" cy="9.5" r="1.5"/>
                <circle fill="#292F33" cx="21.5" cy="9.5" r="1.5"/>
                <path fill={highlight} d="M9.053 21.529c-.14.236-3.053.732-2.303-.731s2.443.497 2.303.731z"/>
                <path fill={highlight}
                      d="M9.891 20.124c-.218.225-3.188.391-1.922-1.404 1.265-1.793 2.234 1.082 1.922 1.404z"/>
                <path fill={highlight}
                      d="M11.657 18.66c-.378.231-3.471-.501-1.407-1.932 1.872-1.296 1.906 1.626 1.407 1.932z"/>
                <path fill={highlight}
                      d="M14.102 17.427c-1.008.299-3.378-1.302-.881-2.141 2.498-.839 1.889 1.842.881 2.141zm12.754 4.102c.141.235 3.053.731 2.303-.731-.75-1.463-2.443.497-2.303.731z"/>
                <path fill={highlight}
                      d="M26.019 20.124c.218.225 3.188.391 1.922-1.404-1.266-1.793-2.235 1.082-1.922 1.404z"/>
                <path fill={highlight}
                      d="M24.253 18.66c.378.231 3.471-.501 1.406-1.932-1.872-1.296-1.906 1.626-1.406 1.932z"/>
                <path fill={highlight}
                      d="M21.808 17.427c1.008.299 3.378-1.302.881-2.141-2.499-.839-1.89 1.842-.881 2.141z"/>
                <path fill={darker}
                      d="M26.849 16.25c0 .414-2.189-2.25-8.849-2.25-6.661 0-8.848 2.664-8.848 2.25 0-.414 2.754-3.75 8.848-3.75 6.094 0 8.849 3.336 8.849 3.75z"/>
                <path fill={highlight}
                      d="M10.793 24.433c0-.414 1.782 2.25 7.207 2.25s7.208-2.664 7.208-2.25c0 .414-2.244 3.75-7.208 3.75s-7.207-3.336-7.207-3.75z"/>
            </svg>
            <CrabInfo style={{color: darker}}>{info.name} {info.point}pt</CrabInfo>
        </CrabBase>
    )
}

function crabPosition(x: number, y: number, gameFieldSize: number): { left: string, top: string } {
    const unit = 80 / gameFieldSize;
    const gap = 10 / gameFieldSize;
    const left = unit * x - gap
    const top = unit * y - gap
    return {
        left: `${left}vh`,
        top: `${top}vh`,
    }
}

function direction2rotate(direction: Crab["direction"]) {
    switch (direction) {
        case "N":
            return 0;
        case "E":
            return 90;
        case "S":
            return 180;
        case "W":
            return 270;
    }
}

const CrabBase = styled.div<{ $gameFieldSize: number; }>`
    position: absolute;
    width: ${props => 100 / props.$gameFieldSize}vh;
    height: ${props => 100 / props.$gameFieldSize}vh;
    transition: left 0.6s, top 0.6s;
`;

const CrabInfo = styled.div`
    position: absolute;
    top: 3.2vh;
    left: 0;
    font-size: 0.8rem;
    font-weight: bold;
    white-space: nowrap;
`;

export default GameField
