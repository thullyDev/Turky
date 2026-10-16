export function frameIndex(delays: number[], started: number, now: number): number {
    let cycle = 0;

    for (const delay of delays) {
        cycle += delay;
    }

    if (cycle <= 0 || delays.length === 0) {
        return 0;
    }

    let time = (now - started) % cycle;

    for (let index = 0; index < delays.length; index++) {
        const delay = delays[index];

        if (time < delay) {
            return index;
        }

        time -= delay;
    }

    return delays.length - 1;
}
