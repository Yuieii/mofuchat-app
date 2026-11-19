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
