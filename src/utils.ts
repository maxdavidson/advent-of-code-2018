import fs from 'fs';
import readline from 'readline';
import stream from 'stream';
import util from 'util';

const readFile = util.promisify(fs.readFile);

export function readFileAsText(path: string) {
  return readFile(path, { encoding: 'utf8' });
}

export function readLines(path: string): AsyncIterable<string> {
  const lineReader = readline.createInterface({
    input: fs.createReadStream(path),
  });

  // Async iteration support was added to readline in
  // https://github.com/nodejs/node/commit/2a7432dadec08bbe7063d84f1aa4a6396807305c
  if (Symbol.asyncIterator in lineReader) {
    // @ts-ignore
    return lineReader;
  }

  const readableStream = new stream.Readable({
    objectMode: true,
    read() {
      lineReader.resume();
    },
    destroy(err, cb) {
      lineReader.removeListener('line', handleLine);
      lineReader.removeListener('close', handleClose);
      lineReader.close();
      cb(err);
    },
  });

  function handleLine(input: string) {
    // Pause the line reader if we can't push any more lines
    if (!readableStream.push(input)) {
      lineReader.pause();
    }
  }

  function handleClose() {
    // Close the readable stream by pushing an EOF-signaling `null`
    readableStream.push(null);
  }

  lineReader.addListener('line', handleLine);
  lineReader.addListener('close', handleClose);

  return readableStream;
}

export async function drain<T>(asyncIterable: AsyncIterable<T>): Promise<Array<T>> {
  const items = [] as T[];

  for await (const item of asyncIterable) {
    items.push(item);
  }

  return items;
}
