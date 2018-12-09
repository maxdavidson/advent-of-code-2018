// A circular doubly-linked list
class Marble<T> {
  prev: Marble<T> = this;
  next: Marble<T> = this;

  constructor(readonly value: T) {}

  push(value: T) {
    const marble = new Marble(value);
    marble.prev = this;
    marble.next = this.next;
    this.next.prev = marble;
    this.next = marble;
    return marble;
  }

  pop() {
    this.prev.next = this.next;
    this.next.prev = this.prev;
    return this.value;
  }

  *[Symbol.iterator]() {
    let current: Marble<T> = this;
    do {
      yield current.value;
    } while ((current = current.next) !== this);
  }
}

export function maxScore(playerCount: number, lastMarbleNumber: number): number {
  const playerScores = new Uint32Array(playerCount);
  let currentMarble = new Marble(0);

  for (let marbleNumber = 1; marbleNumber <= lastMarbleNumber; ++marbleNumber) {
    if (marbleNumber % 23 === 0) {
      currentMarble = currentMarble.prev.prev.prev.prev.prev.prev.prev;
      playerScores[marbleNumber % playerCount] += marbleNumber + currentMarble.pop();
      currentMarble = currentMarble.next;
    } else {
      currentMarble = currentMarble.next.push(marbleNumber);
    }
  }

  return playerScores.reduce((a, b) => Math.max(a, b));
}
