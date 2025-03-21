import { createSignal } from "solid-js";
import { BACKGROUND_SVG } from "../components/home-bg";
import { A } from "@solidjs/router";

export default function login() {
  return (
    <div class="flex justify-between w-screen h-[100vh] z-1 px-10 items-center">
      <div class="flex flex-col w-[25%] h-[85%] border-white/20 border-[1px] backdrop-blur-[25px] bg-[#1F1F1F]/50 rounded-[8px] gap-4">
        <h1 class="text-white text-2xl text-center py-3.5 border-b-[1px] border-white/20">
          Login
        </h1>
        <form class="flex flex-col px-4 gap-y-4 scheme-dark">
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Email/Username</label>
            <input
              placeholder="e.g JohnD or example@domain.ext"
              required
              type="username or email"
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Password</label>
            <input
              placeholder="strong password"
              type="password"
              required
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <button class="mt-4 rounded-[8px] text-white border-[1px] border-black/50 bg-[#FF4400] hover:bg-[#ff5f25] hover:text-[#cccccc] hover:cursor-pointer duration-300 px-4 py-2">
            Login
          </button>
        </form>
      </div>
      <p class="text-white text-6xl font-light font-[Outfit]">OR</p>
      <div class="flex flex-col w-[25%] h-[85%] border-white/20 border-[1px] backdrop-blur-[25px] bg-[#1F1F1F]/50 rounded-[8px] gap-4">
        <h1 class="text-white text-2xl text-center py-3.5 border-b-[1px] border-white/20">
          Create Account
        </h1>
        <form class="flex flex-col px-4 gap-y-4 scheme-dark">
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Username*</label>
            <input
              placeholder="e.g JohnD"
              required
              type="username"
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Email*</label>
            <input
              placeholder="e.g exampl@domain.ext"
              required
              type="email"
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Password*</label>
            <input
              placeholder="strong password"
              type="password"
              required
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <div class="flex flex-col col-start-1 col-span-1 row-start-1 gap-2">
            <label class="text-white">Confirm Password*</label>
            <input
              placeholder="strong password"
              type="password"
              required
              class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
            ></input>
          </div>
          <button class="mt-4 rounded-[8px] text-white border-[1px] border-black/50 bg-[#FF4400] hover:bg-[#ff5f25] hover:text-[#cccccc] hover:cursor-pointer duration-300 px-4 py-2">
            Sign Up
          </button>
        </form>
      </div>
    </div>
  );
}
