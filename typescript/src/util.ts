import fs from 'fs';
import readline from 'readline';

/**
 * read lines from a file into an array of string
 * @param path
 */
function readLines(path: string): Promise<string[]> {

    return new Promise((resolve, reject) => {
        const lines: string[] = [];


        const readInterface = readline.createInterface(
            {
                input: fs.createReadStream(path, "utf8")
            });

        readInterface.on('line', function (line: string) {
            lines.push(line);
        });

        readInterface.on('close', function () {
            return resolve(lines);
        })

    });
}

/**
 * Synchronously read the ENTIRE file located at path into an array of file lines. Each element in the array will be
 * a line of the file. The newline characters are NOT included in the array.
 * @param path
 */
function readFile(path: string): string[] {
    const fileStr = fs.readFileSync(path, 'utf-8');
    const lines = fileStr.split(/\r\n|\r|\n/);
    console.log(lines);
    return lines;
}

export { readLines, readFile };