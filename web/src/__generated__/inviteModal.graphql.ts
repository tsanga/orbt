/**
 * @generated SignedSource<<394304d5cd431a02985c24b2c12330ba>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { Fragment, ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type inviteModal$data = {
  readonly name: string;
  readonly " $fragmentType": "inviteModal";
};
export type inviteModal$key = {
  readonly " $data"?: inviteModal$data;
  readonly " $fragmentSpreads": FragmentRefs<"inviteModal">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "inviteModal",
  "selections": [
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "name",
      "storageKey": null
    }
  ],
  "type": "Room",
  "abstractKey": null
};

(node as any).hash = "255d3abab9acb8158f91d80460b814eb";

export default node;
