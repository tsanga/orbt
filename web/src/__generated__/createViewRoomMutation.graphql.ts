/**
 * @generated SignedSource<<eea4337611580fe6991c8274f4b24d55>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Mutation } from "relay-runtime";
export type createViewRoomMutation$variables = {
  name?: string | null;
};
export type createViewRoomMutation$data = {
  readonly createRoom: {
    readonly createToken: {
      readonly token: string | null;
    };
    readonly id: string;
    readonly name: string;
  };
};
export type createViewRoomMutation = {
  response: createViewRoomMutation$data;
  variables: createViewRoomMutation$variables;
};

const node: ConcreteRequest = (function () {
  var v0 = [
      {
        defaultValue: null,
        kind: "LocalArgument",
        name: "name",
      },
    ],
    v1 = [
      {
        alias: null,
        args: [
          {
            kind: "Variable",
            name: "name",
            variableName: "name",
          },
        ],
        concreteType: "Room",
        kind: "LinkedField",
        name: "createRoom",
        plural: false,
        selections: [
          {
            alias: null,
            args: null,
            kind: "ScalarField",
            name: "id",
            storageKey: null,
          },
          {
            alias: null,
            args: null,
            kind: "ScalarField",
            name: "name",
            storageKey: null,
          },
          {
            alias: null,
            args: null,
            concreteType: "Token",
            kind: "LinkedField",
            name: "createToken",
            plural: false,
            selections: [
              {
                alias: null,
                args: null,
                kind: "ScalarField",
                name: "token",
                storageKey: null,
              },
            ],
            storageKey: null,
          },
        ],
        storageKey: null,
      },
    ];
  return {
    fragment: {
      argumentDefinitions: v0 /*: any*/,
      kind: "Fragment",
      metadata: null,
      name: "createViewRoomMutation",
      selections: v1 /*: any*/,
      type: "Mutation",
      abstractKey: null,
    },
    kind: "Request",
    operation: {
      argumentDefinitions: v0 /*: any*/,
      kind: "Operation",
      name: "createViewRoomMutation",
      selections: v1 /*: any*/,
    },
    params: {
      cacheID: "a57d9ec0322ef256cf3b175ad5998831",
      id: null,
      metadata: {},
      name: "createViewRoomMutation",
      operationKind: "mutation",
      text: "mutation createViewRoomMutation(\n  $name: String\n) {\n  createRoom(name: $name) {\n    id\n    name\n    createToken {\n      token\n    }\n  }\n}\n",
    },
  };
})();

(node as any).hash = "4850f403c841a51c313918ce07d75a12";

export default node;
