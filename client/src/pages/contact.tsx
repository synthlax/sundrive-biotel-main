import { createEffect, Suspense } from "solid-js";
import { Card } from "../components/card";
import { Field } from "../components/field";
import { Button } from "../components/button";
import { A } from "@solidjs/router";

export default function Contact() {
  return (
    <div class="flex flex-col w-screen h-full top-[10%] px-4 items-center gap-y-2.5">
      <h1 class="text-white/50 font-light text-2xl mt-2.5">
        Contact and Information
      </h1>
      <Card padding="large">
        <div class="md:flex flex-row gap-x-2.5">
          <div class="md:flex flex-col gap-y-2.5">
            <Card padding="default">
              <h1 class="text-white font-normal">Company Operations Contact</h1>
              <h2 class="text-white font-extralight">Barnett Arfa</h2>
              <h2 class="text-white font-extralight underline">
                corporate@sundrive.company
              </h2>
            </Card>
            <Card padding="default">
              <h1 class="text-white font-normal">Primary Maintainer</h1>
              <h2 class="text-white font-extralight">Angel Alvidrez</h2>
              <h2 class="text-white font-extralight underline">
                jose.alvidreztorres@gmail.com
              </h2>
            </Card>
            <Card padding="default">
              <h1 class="text-white font-normal">Contributors</h1>
              <h2 class="text-white font-extralight">TBD</h2>
              <h2 class="text-white font-extralight underline">N/A</h2>
            </Card>
          </div>
          <div class="md:flex flex-col gap-2.5 z-2 isolate">
            <form>
              <div class="md:flex flex-row gap-y-4 gap-x-1">
                <Field title="Name" placeholder="e.g John Doe"></Field>
                <Field title="Email" placeholder="example@domain.ext" ></Field>
              </div>
              <textarea placeholder="Text here..." class="h-[25vh] w-[100%] text-white/15 font-mono p-4 outline-0 border-[1px] border-white/4 rounded-lg bg-neutral-700/20 hover:text-white/50 hover:border-white/50 duration-300 focus:border-white resize-none z-2" required></textarea>
              <Button variant="alt" size="default" class="w-full">
                <A href="/sumbit" class="height-[100%] w-[100%] absolute">
                  Submit
                </A>
              </Button>
            </form>
          </div>
        </div>
      </Card>
    </div>
  );
}
