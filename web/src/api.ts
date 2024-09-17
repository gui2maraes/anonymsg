import {DefaultApi, Message, PublicJwk, ResponseError} from './oapi';
import {KeyPair} from './KeyPair';
import { EncryptedContent } from './KeyPair';

const Api = new DefaultApi();

/**
 * fetches messages sent to a recipient, with given limit.
 * @returns {(Message[]|undefined)} list of messages, or `undefined` if recipient not found
 */
export async function getMessagesTo(recipient: string, limit?: number): Promise<Message[] | undefined> {
    try {
        return await Api.apiMessagesGet({recipient, limit});
    } catch (e) {
        if (e instanceof ResponseError) {
            return undefined;
        }
        throw e;
    }

}

export enum RegisterAliasError {
    AlreadyExists,
}
/**
 * registers an alias and key
 * @returns {(void|RegisterAliasError)} error if alias already exists
 */
export async function registerAlias(keyPair: KeyPair): Promise<void | RegisterAliasError> {
    try {
        await Api.apiRegisterPost({
            registerRequest: {
                alias: keyPair.alias,
                publicKey: await keyPair.publicJwk()
            }
        });
    } catch (e) {
        if (e instanceof ResponseError) {
            if (e.response.status === 409) {
                return RegisterAliasError.AlreadyExists;
            }
        }
        throw e;
    }
}

/// fetches list of similar aliases, ordered by similarity
export async function searchAlias(alias: string): Promise<string[]> {
    return await Api.apiSearchAliasGet({alias});
}
/// fetches a key associated with an alias
/// returns `undefined` if alias is not found
export async function fetchAliasKey(alias: string): Promise<PublicJwk | undefined> {
    try {
        return await Api.apiRegistryAliasGet({alias});
    } catch (e) {
        if (e instanceof ResponseError) {
            return undefined;
        }
        throw e;
    }
}
export enum PublishMessageError {
    RecipientDoesNotExist
}
/// publishes a message to a given recipient
/// returns error if recipient does not exist
export async function publishMessage(recipient: string, content: EncryptedContent): Promise<void | PublishMessageError> {
    try {
        await Api.apiPublishPost({publishMessage: {content: content.base64, recipient}});
    } catch (e) {
        if (e instanceof ResponseError) {
            if (e.response.status == 404) {
                return PublishMessageError.RecipientDoesNotExist;
            }
        }
        throw e;
    }
}
