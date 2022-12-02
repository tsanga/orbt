/**
 * @generated SignedSource<<d00d0e01c6e80efbd8701fac89df70f0>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { Fragment, ReaderFragment } from 'relay-runtime';
import { FragmentRefs } from "relay-runtime";
export type roomTopBarTitle$data = {
  readonly name: string;
  readonly " $fragmentType": "roomTopBarTitle";
};
export type roomTopBarTitle$key = {
  readonly " $data"?: roomTopBarTitle$data;
  readonly " $fragmentSpreads": FragmentRefs<"roomTopBarTitle">;
};

const node: ReaderFragment = {
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "roomTopBarTitle",
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

(node as any).hash = "676611e69b2ce03dddbafb367dd41b0d";

export default node;
