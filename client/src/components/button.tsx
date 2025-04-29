import { cva, VariantProps } from "class-variance-authority";
import { Component, JSX } from "solid-js";
import { twMerge } from 'tailwind-merge';

const buttonVariants = cva("px-4 py-2 rounded font-mono", {
    variants: {
        variant: {
            default: "flex justify-center items-center px-4 py-2 rounded bg-white border border-black/30 text-black hover:bg-[#007aff] hover:text-white hover:cursor-pointer duration-300",
            action: "flex justify-center items-center px-4 py-2 rounded bg-[#007aff] border border-black/30 text-white hover:bg-[#439DFF] hover:shadow-[0px_0px_120px_rgba(67,157,255,0.25)] hover:cursor-pointer duration-300",
            alt: "flex justify-center items-center px-4 py-2 rounded border border-[#999] text-[#999] hover:bg-white hover:text-black hover:cursor-pointer duration-300",
            destructive: "flex justify-center items-center px-4 py-2 rounded bg-[#ff1e00] border border-black/30 text-white hover:bg-[#FF6652] hover:shadow-[0px_0px_120px_rgba(255,30,0,0.25)] hover:cursor-pointer duration-300",
            navigation: "flex gap-2.5 items-center px-4 py-2 text-white hover:cursor-pointer"
        },
        size: {
            default: "h-12",
            medium: "h-24",
            large: "h-52"
        }
    },
    defaultVariants: {
        variant: "default",
        size: "default"
    }
})

export const Button: Component<{type: string, variant: string, size: string, children: JSX.Element}> & VariantProps<typeof buttonVariants>  = ({ type, variant, size, className, children, ...props}: any ) => {
    return(
        <button type={type} class={twMerge(buttonVariants({ variant, size, className, ...props }))}>
            {children}
        </button>
    )
}