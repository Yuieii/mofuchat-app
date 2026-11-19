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
}

export interface MofuTwitchCommentData extends MofuCommentData { 

}



export interface MofuComment<K extends keyof MofuServiceList = keyof MofuServiceList, KK extends keyof MofuServiceList = K extends infer A ? A extends keyof MofuServiceList ? A : never : never> { 
    service: KK;
    data: MofuServiceList[KK];
    color: RGBColor,
    id: string;
    authorAvatar: string;
    authorDisplayName: string;
}

export type MofuEvent<K extends keyof MofuEventTypeLookup = keyof MofuEventTypeLookup> =
    K extends infer A ? A extends keyof MofuEventTypeLookup ? { event: A, data: MofuEventTypeLookup[A] } : never : never;