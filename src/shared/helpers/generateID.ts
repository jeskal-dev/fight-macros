class Snowflake {
  private static readonly EPOCH = 1_600_000_000_000;
  private static lastTs = 0;
  private static seq = 0;

  static next(): number {
    const now = Date.now();
    if (now === this.lastTs) {
      this.seq = (this.seq + 1);
      if (this.seq === 0) {
        while (Date.now() <= now) console.log("SPINNING");
        return this.next();
      }
    } else {
      this.seq = 0;
      this.lastTs = now;
    }

    const ts = now - this.EPOCH;
    const seq = this.seq;

    return ts | seq;
  }
}

export function generateID() {
  return Snowflake.next();
}
