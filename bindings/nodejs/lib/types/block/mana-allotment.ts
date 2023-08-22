// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountId } from "..";

/**
 * An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
 * in the form of Block Issuance Credits to the account.
 */
class ManaAllotment {
  /**
   * The Account to allot Mana to.
   */
  readonly accountId: AccountId;
  /**
   * The Amount of Mana to allot.
   */
  readonly mana: bigint;

  constructor(accountId: AccountId, mana: bigint) {
    this.accountId = accountId;
    this.mana = mana;
  }
}

export {
  ManaAllotment
};

