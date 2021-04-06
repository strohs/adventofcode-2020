const fs = require("fs");

// safe with negative numbers unlike JS % operator
const absoluteModulo = (a, b) => ((a % b) + b) % b;

// returns x where (a * x) % b == 1
// https://rosettacode.org/wiki/Modular_inverse
const getInverse = (a, mod) => {
    const b = a % mod;
    for (let i = 1; i < mod; i++) {
        if ((b * i) % mod === 1) {
            return i;
        }
    }
    return 1;
};

const chineseRemainderTheorem = (lines) => {
    // x =- a (mod n)
    // x - some unknown, constant value of t
    // a - bus number MINUS offset % bus number
    // n - cycle length (= bus number)

    // to solve each row, we also need
    // N - all n's added up
    // nU = N / n
    // i - inverse modulo

    // multiply all busIDs together and store them in N
    const N = lines.reduce((acc, cur) => {
        if (cur === "x") {
            return acc;
        }
        return acc === null ? cur : acc * cur;
    }, null);

    const sum = lines.reduce((acc, cur, idx) => {
        if (cur === "x") {
            return acc;
        }
        const a = absoluteModulo(cur - idx, cur);
        const nU = N / cur;
        const inverse = getInverse(nU, cur);
        console.log(`x = ${a} (mod ${cur})`);
        return acc + BigInt(BigInt(a) * BigInt(nU) * BigInt(inverse));
    }, 0n);

    return sum % BigInt(N);
};

const findMatchingT = (buses) => {
    let busesInt = buses.map((bus) => (bus === "x" ? "x" : parseInt(bus, 10)));
    return chineseRemainderTheorem(busesInt);
};

fs.readFile("../input/input13.txt", "utf8", (err, data) => {
    const lines = data.split("\n");
    const timestamp = parseInt(lines[0], 10);
    const buses = lines[1].split(",");
    const delays = buses
        .filter((bus) => bus !== "x")
        .map((bus) => ({ bus: bus, delay: bus - (timestamp % parseInt(bus, 10)) }))
        .sort((a, b) => a.delay - b.delay);
    console.log(
        `The shortest delay is ${delays[0].delay} minutes on bus ${delays[0].bus}.`
    );
    console.log(`The desired pattern occurs at ${findMatchingT(buses)}.`);
});