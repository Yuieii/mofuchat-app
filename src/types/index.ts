import { type Comment } from "@onecomme.com/onesdk/types/Comment";

export type Prettify<T> = {
    [K in keyof T]: T[K] extends infer V ? V : never;
};

export type Constructor = {
    [Symbol.hasInstance](value: any): boolean
};

export type ConstructorOf<C, Args extends any[] = any[]> = {
    new(...args: Args): C,
    [Symbol.hasInstance](value: any): boolean
};

export type RequiredKeys<T> = {
    [K in keyof T]-?: {} extends Pick<T, K> ? never : K
}[keyof T];

export type CommentStyle = Record<string, string>;

export type CommentDataHas<K extends string> = Comment extends infer C
    ? C extends { data: infer D }
        ? K extends keyof D
            ? D & Required<Pick<C["data"], K>> : never
        : never
    : never;

export type CommentHas<K extends string> = Comment & { data: CommentDataHas<K> };

export type CommentOf<T extends Comment["service"]> = Comment extends infer C
    ? C extends { service: infer S }
        ? T extends S ? C : never
        : never
    : never;

export type Split<S extends string, D extends string = ""> = //string extends S ? string[] :
    S extends `${infer A}${D}${infer B}`
        ? B extends "" ? D extends "" ? [A] : [A, ""] : [A, ...Split<B, D>]
        : [S];
        
export type Replace<S extends string, R extends string, T extends string> = //string extends S ? string :
    S extends `${infer A}${R}${infer B}`
        // Don't need to process A. It is impossible for A to include R due to how string type inferring works
        ? `${A}${T}${Replace<B, R, T>}`
        : S;

// type Concat<T extends string[]> = T extends [infer S, ...infer R]
//     ? S extends string
//         ? R extends string[]
//             ? `${S}${Concat<R>}` 
//             : never 
//         : never
//     : T extends { length: 0 } ? "" : string; // Runtime fallback

export type Concat<T extends string[]> = Join<T>;

export type Join<T extends string[], D extends string = ""> = T extends [infer S, ...infer R]
    ? S extends string
        ? R extends string[]
            ? `${S}${T extends { length: 1 } ? "" : D}${Join<R, D>}` 
            : never 
        : never
    : T extends { length: 0 } ? "" : string; // Runtime fallback

export type CapitalizeEach<T extends string[]> = T extends [infer S, ...infer R]
    ? S extends string
        ? R extends string[]
            ? [Capitalize<S>, ...CapitalizeEach<R>]
            : never
        : never
    : T extends { length: 0 } ? [] : string[]; // Runtime fallback

export type CapitalizeEachAfterFirst<T extends string[]> = T extends [infer S, ...infer R]
    ? S extends string
        ? R extends string[]
            ? [S, ...CapitalizeEach<R>]
            : never
        : never
    : T extends { length: 0 } ? [] : string[]; // Runtime fallback

export type UppercaseLetters = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]; // Split<"ABCDEFGHIJKLMNOPQRSTUVWXYZ">;

export type LowercaseLetters = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]; // Split<Lowercase<Concat<UppercaseLetters>>>;

export type UppercaseLetter = UppercaseLetters[number];

export type LowercaseLetter = LowercaseLetters[number];

type Undercase_<T extends string> = T extends `${infer A}${infer B}`
    ? A extends UppercaseLetter ? `_${Lowercase<A>}${Undercase_<B>}` : `${A}${Undercase_<B>}`
    : T;

export type SnakeCase<T extends string> = Undercase_<T> extends `${infer U}`
    ? U extends `_${infer S}` ? S : U
    : T;

export type PascalCase<T extends string> = Concat<CapitalizeEach<Split<T, "_">>>;

export type CamelCase<T extends string> = Concat<CapitalizeEachAfterFirst<Split<T, "_">>>;

export type ShoutingCase<T extends string> = Uppercase<SnakeCase<T>>;

export type KebabCase<T extends string> = Replace<SnakeCase<T>, "_", "-">;