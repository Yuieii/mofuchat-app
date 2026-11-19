const primaryForeground = "#fff";
const primaryBackground = "hsl(330, 29%, 53%)";
const secondaryBackground = "hsla(330, 29%, 53%, 0.5)";

interface LoggerInterface {
    log(...data: unknown[]): void;
    warn(...data: unknown[]): void;
    error(...data: unknown[]): void;
}

export const Log = {
    createDummy(): LoggerInterface {
        return new Proxy(console, {
            get(target, p, receiver) {
                return () => { };
            },
        });
    },

    createLogger(name: string): LoggerInterface {
        if (location.hostname.endsWith("yuieii.com")) {
            return this.createDummy();
        }

        const format = `%c[%c${name}%c]`;
        const prepend = [
            `background: ${secondaryBackground}; color: transparent;`,
            `background: ${primaryBackground}; color: ${primaryForeground}; font-weight: bold; padding-inline: 0.5em;`,
            `background: ${secondaryBackground}; color: transparent;`
        ];

        return {
            log: console.log.bind(console, format, ...prepend),
            warn: console.warn.bind(console, format, ...prepend),
            error: console.error.bind(console, format, ...prepend),
        };
    },
}