import { Prettify } from "../../types";
import { RGBColor, Service } from "../config";

export interface MofuAppEventTypeLookup {
    commentCleared: null,
    commentDeleted: { id: string },
}

export interface MofuEventTypeLookup {
    serviceConnected: { id: string };
    serviceDisconnected: { id: string };
    serviceDeactivated: { id: string };
    commentCreated: MofuComment,
    requiresSignIn: {
        platform: keyof MofuServiceList,
        code?: string,
        url: string,
        expiresIn: number
    }
}

export interface MofuServiceList { 
    twitch: MofuTwitchCommentData
}

export type MofuServiceVariant = keyof MofuServiceList;

export interface MofuCommentData {
    id: string;
    profileImage: string;
    name: string;
    displayName: string;
    badges: { url: string, label: string }[];
    renderers: MofuCommentRenderer[];
    timestamp: string;
    replyCommentId?: string;
}

export interface MofuTwitchCommentData extends MofuCommentData { 
    bits?: number;
}

export interface MofuComment<
    K extends keyof MofuServiceList = keyof MofuServiceList,
    KK extends keyof MofuServiceList = K extends infer A ? A extends keyof MofuServiceList ? A : never : never
> { 
    service: KK;
    data: MofuServiceList[KK];
    color: RGBColor,
    id: string;
}

export interface MofuCommentTextRenderer {
    type: "text";
    content: string;
}

export interface MofuCommentEmoteRenderer { 
    type: "emote";
    name: string;
    url: string;
}

export interface MofuCommentMentionRenderer { 
    type: "mention";
    content: string;
}

export type MofuCommentRenderer =
    | MofuCommentTextRenderer
    | MofuCommentEmoteRenderer
    | MofuCommentMentionRenderer;

type MofuEventData<K> = K extends infer A
    ? A extends keyof MofuEventTypeLookup
        ? MofuEventTypeLookup[A]
        : unknown
    : never;

export type MofuEvent<K extends string = string> = Prettify<K extends any
    ? { event: K, data: MofuEventData<K> }
    : { event: string, data: unknown }>;

type MofuAppEventData<K> = K extends infer A
    ? A extends keyof MofuAppEventTypeLookup
        ? MofuAppEventTypeLookup[A]
        : unknown
    : never;

export type MofuAppEvent<K extends string = string> = Prettify<K extends any
    ? { event: K, data: MofuAppEventData<K> }
    : { event: string, data: unknown }>;