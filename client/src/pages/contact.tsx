import { createEffect, Suspense } from "solid-js";

export default function Contact() {
  return (
    <div class="flex flex-col w-screen h-dvh bg-linear-to-b from-[#100400] to-50% to-[#000000] top-[10%] px-4 items-center ">
      <div class="flex flex-col w-screen scheme-dark gap-y-4">
        <h1 class="text-xl text-white px-[2%] py-4 bg-[#1e1e1e]/50 border-b-1">
          Contact & Acknowledgements
        </h1>

        <ul class="flex mt-4 text-white px-[2%] list-none flex-col overflow-hidden">
          <li class="font-bold">
            Company Email:{" "}
            <span class="underline text-orange-500 font-normal">
              corporate@sundrive.company
            </span>
          </li>
          <li class="font-bold">Company Operations Contact: <span class="font-normal">Barnett Arfa Midjaja</span></li>
          <li class="font-bold">Active Maintainer: <span class="font-normal">Angel Alvidrez-Torres</span></li>
          <li class="font-bold">Contributors: <span class="font-normal"></span></li>
        </ul>
        <form class="">
          <div class="px-[2%] grid grid-cols-2 grid-rows-2 mt-2.5 gap-4">
            <div class="flex flex-col col-start-1 col-span-1 row-start-1">
              <label class="text-white">Full Name</label>
              <input
                placeholder="e.g John J. Doe"
                required
                class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
              ></input>
            </div>
            <div class="flex flex-col col-start-2 col-span-1 row-start-1">
              <label class="text-white">Phone Number</label>
              <input
                placeholder="e.g John J. Doe"
                required
                class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
              ></input>
            </div>
            <div class="flex flex-col col-start-1 col-span-1 row-start-2">
              <label class="text-white">Subject</label>
              <input
                placeholder="e.g Contribution Request"
                minLength={1}
                required
                class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
              ></input>
            </div>
            <div class="flex flex-col col-start-2 col-span-1 row-start-2">
              <label class="text-white">Email</label>
              <input
                placeholder="example@domain.com"
                type="email"
                required
                class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"
              ></input>
            </div>
          </div>
          <div class="px-[2%] mt-4">
            <label class="text-white">Body</label>
            <textarea
              placeholder="Input here..."
              minLength={1}
              required
              class="w-[100%] h-32 bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 text-pretty break-normal py-2 resize-none invalid:border-red-400"
            ></textarea>
          </div>
          <button class="ml-[2%] mt-2 mb-4 w-[25%] text-white bg-[#FF4400] px-4 py-2 rounded-[4px] hover:text-[#cccccc] hover:bg-[#ff5f25] hover:shadow-[0px_0px_50px_rgba(255,95,37,.5)] duration-300 hover:cursor-pointer">
            Sumbit
          </button>
        </form>
      </div>
    </div>
  );
}
