import { GeneralOptionFlags, CommandOptionReturn, WithArity } from "./utils";
export declare type ArrayFlags<Arity extends number = 1> = GeneralOptionFlags & {
    arity?: Arity;
};
/**
 * Used to annotate array options. Such options will be strings unless they
 * are provided a schema, which will then be used for coercion.
 *
 * @example
 * --foo hello --foo bar
 *     ► {"foo": ["hello", "world"]}
 */
export declare function Array<Arity extends number = 1>(descriptor: string, opts: ArrayFlags<Arity> & {
    required: true;
}): CommandOptionReturn<Array<WithArity<string, Arity>>>;
export declare function Array<Arity extends number = 1>(descriptor: string, opts?: ArrayFlags<Arity>): CommandOptionReturn<Array<WithArity<string, Arity>> | undefined>;
export declare function Array<Arity extends number = 1>(descriptor: string, initialValue: Array<WithArity<string, Arity>>, opts?: Omit<ArrayFlags<Arity>, 'required'>): CommandOptionReturn<Array<WithArity<string, Arity>>>;
