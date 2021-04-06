
import { readLines, readFile } from "./util";
import path from "path";

const filePath = path.join(__dirname, '..', 'input', 'input13.txt');

async function parseInput(path: string): Promise<[number, number[]]> {
    try {
        // const lines = await readLines(path);
        const lines = readFile(path);
        const earliestTimestamp = parseInt(lines[0]);
        const busIDs: number[] = lines[1]
            .split(',')
            .filter(id => Number.parseInt(id))
            .map(idNum => parseInt(idNum));
        return [earliestTimestamp, busIDs];
    } catch (err) {
        throw err;
    }
}

// returns a Promise tuple containing: [earliestBusID, earliestTimestamp]
async function prob13(filePath: string): Promise<[number, number, number]> {
    // parse input files into numbers
    let [earliestTs, busIDs] = await parseInput(filePath);
    busIDs.sort();

    // input should contain at least one bus that leaves at right timestamp
    for (let currTs = earliestTs; currTs < Number.MAX_SAFE_INTEGER; currTs++) {
        for (const busID of busIDs) {
            //console.log(`${busID} ${currTs}`);
            if (currTs % busID === 0) {
                return [earliestTs, busID, currTs];
            }
        }
    }
    throw Error('no answer was found but there should be one')
}


prob13(filePath)
    .then(([arrivalTs, busID, busDepartureTs]) => {
        console.log(`${busID} at ${busDepartureTs} and we arrived at ${arrivalTs}`);

        const answer = (busDepartureTs - arrivalTs) * busID;
        console.log(answer);
    })