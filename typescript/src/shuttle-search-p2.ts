import {readFile} from "./util";
import path from "path";

//// Advent of Code 2020: Shuttle Search Part2

const filePath = path.join(__dirname, '..', 'input', 'input13.txt');

// BusData holds the bus Ids in a Map, plus the first busId and last busId
interface BusData {
    // a Map from:   BusId => array index offset
    map: Map<number, number>,
    // the first busId in the original input array, and first in the map
    first: number,
    // the last busId in the original input array, and last in the map
    last: number,
}

/**
 * parse the comma separated list of busId(s) into a BusData object
 * @param path
 */
function parseInput(path: string): BusData {
    try {
        const offsetMap: Map<number, number> = new Map();
        let firstId = -1;
        let lastId = -1;
        const busIds = readFile(path)[1].split(',');
        //const busIds = ['67','7','x','59','61'];

        busIds.forEach((id, idx) => {
                const busId = Number.parseInt(id, 10);
                if (busId) {
                    if (idx === 0) firstId = busId;
                    if (idx === busIds.length - 1) lastId = busId;
                    offsetMap.set(busId, idx);
                }
            });

        return {
            map: offsetMap,
            first: firstId,
            last: lastId,
        }
    } catch (err) {
        throw err;
    }
}

/**
 * find the earliest timestamp that all buses can depart from
 * @param busData
 */
function earliestTimestamp(busData: BusData): number {
    // store the busIds from the busData.map in an array
    const idArr = Array.from(busData.map.keys());

    // find the maximum busId in the busData.map
    function maxKey() {
        return Math.max(...idArr);
    }

    function allBusesCanDepart(timestamp: number): boolean {
        // holds the current timestamp offset from 'timestamp'
        let tsOffset = timestamp;

        // iterate thru all busId, checking if they can divide the current timestamp.
        for (const busId of idArr) {
            // increment the current timestamp, by the array index offset of the current busId
            tsOffset = timestamp + busData.map.get(busId)!;
            if (tsOffset % busId !== 0) {
                return false;
            }
        }

        // // check if the first busId can divide the last timestamp
        // if (tsOffset % busData.first !== 0) {
        //     return false;
        // }
        return true;
    }

    // start iterating from the maximum busId value
    const largest = maxKey();
    const largestOffset = busData.map.get(largest)!;
    let currLargest = largest;
    let timestamp = currLargest;

    // there should be at least one timestamp that satisfies the challenge, so this infinite loop should return
    while (true) {
        if (allBusesCanDepart(timestamp)) {
            return timestamp;
        }
        currLargest += largest;
        timestamp = currLargest - largestOffset;

    }
}

const busData = parseInput(filePath);
console.log(busData);
console.log(earliestTimestamp(busData) );