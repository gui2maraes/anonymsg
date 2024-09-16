/* tslint:disable */
/* eslint-disable */
/**
 * BlindChannel REST API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface Message
 */
export interface Message {
    /**
     * 
     * @type {string}
     * @memberof Message
     */
    content: string;
    /**
     * 
     * @type {Date}
     * @memberof Message
     */
    sentAt: Date;
}

/**
 * Check if a given object implements the Message interface.
 */
export function instanceOfMessage(value: object): value is Message {
    if (!('content' in value) || value['content'] === undefined) return false;
    if (!('sentAt' in value) || value['sentAt'] === undefined) return false;
    return true;
}

export function MessageFromJSON(json: any): Message {
    return MessageFromJSONTyped(json, false);
}

export function MessageFromJSONTyped(json: any, ignoreDiscriminator: boolean): Message {
    if (json == null) {
        return json;
    }
    return {
        
        'content': json['content'],
        'sentAt': (new Date(json['sentAt'])),
    };
}

export function MessageToJSON(value?: Message | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'content': value['content'],
        'sentAt': ((value['sentAt']).toISOString()),
    };
}

