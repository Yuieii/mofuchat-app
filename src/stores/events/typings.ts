import { RGBColor, Service } from "../config";

export interface MofuEventTypeLookup {
    serviceConnected: { id: string };
    serviceDisconnected: { id: string };
    serviceDeactivated: { id: string };
    commentCreated: MofuComment
}

export interface MofuServiceList { 
    twitch: MofuTwitchCommentData
}

export interface MofuCommentData {
    id: string;
    profileImage: string;
    displayName: string;
    badges: { url: string, label: string }[];
    renderers: MofuCommentRenderer[];
    timestamp: number;
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

export type MofuEvent<K extends keyof MofuEventTypeLookup = keyof MofuEventTypeLookup> =
    K extends infer A ? A extends keyof MofuEventTypeLookup ? { event: A, data: MofuEventTypeLookup[A] } : never : never;