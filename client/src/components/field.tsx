import { Component } from "solid-js";
import { JSX } from "solid-js/h/jsx-runtime";

export const Field: Component<{title: string, pattern: string, placeholder: string, type: string, required: string, error_message: string, minLength: number, maxLength: number}> = (props: any) => {
  return (
    <div class="flex flex-col gap-1 col-start-1 row-start-1">
      <label class="text-white font-mono font-extralight after:content-['*'] after:text-red-400">
        {props.title}
      </label>
      <input
        minlength={props.minLength}
        maxlength={props.maxLength}
        pattern={props.pattern}
        placeholder={props.placeholder}
        type={props.type}
        required={props.required}
        class="peer w-fill bg-neutral-700/20 border-white/4 border-[1px] rounded-md text-white/30 font-mono font-extralight outline-0 hover:border-white/50 focus:border-white duration-300 focus:text-white invalid:[&:not(:placeholder-shown):not(:focus)]:border-red-400 invalid:[&:not(:placeholder-shown):not(:focus)]:text-red-400 invalid:focus:text-amber-400 invalid:focus:border-amber-400 px-2.5 py-2 z-2"
      ></input>
      <p class="flex w-fill invisible peer-[&:not(:placeholder-shown):not(:focus):invalid]:visible text-red-400 font-mono font-extralight text-sm text-wrap">
      {props.error_message}
      </p>
    </div>
  );
};
