// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaWalletMobile } from '../index'

const {
    initLogger,
    sendMessage,
    messageHandlerNew,
    listen,
    destroy,
} = IotaWalletMobile

const sendMessageAsync = async (message: string, handler: number): Promise<string> => {
    const { result } = await sendMessage({ message, handler })
    if (JSON.parse(result)?.type === 'error') {
        return Promise.reject(result)
    }
    return result
}

export {
    IotaWalletMobile,
    initLogger as internalInitLogger,
    sendMessageAsync,
    messageHandlerNew,
    listen,
    destroy,
}
