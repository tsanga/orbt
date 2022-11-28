import { useEffect, useContext, useRef } from "react";
import { AppContext, AppDispatchContext } from "@/context";
import { useMutation, graphql } from "react-relay";
import type { useAuthCreateUserMutation as CreateEmptyUserMutation } from "@gql/useAuthCreateUserMutation.graphql";
import { User } from "@domain/models";
import { AuthUser } from "@/context";

export type Auth = {
  user: AuthUser | null;
  isLoggedIn: boolean;
};

export default function useAuth(): Auth {
  const state = useContext(AppContext);
  const dispatch = useContext(AppDispatchContext);
  const ref = useRef(false);

  const [commitMutation, _] = useMutation<CreateEmptyUserMutation>(
    graphql`
      mutation useAuthCreateUserMutation($name: String) {
        createUser(name: $name) {
          id
          name
          token {
            token
          }
        }
      }
    `
  );

  const user = state.user;

  useEffect(() => {
    if (!state.user) {
      if (!ref.current) {
        commitMutation({
          variables: { name: "alex adewole" },
          onCompleted: (response) => {
            if (response.createUser) {
              dispatch({
                type: "SET_USER",
                user: response.createUser as AuthUser,
              });
            }
          },
          onError: (err) => {
            console.log(err);
          },
        });

        ref.current = true;
      }
    }
  }, []);

  return { user, isLoggedIn: !!user };
}
