<script lang="ts">
  import { Avatar, AvatarFallback, AvatarImage } from '$lib/components/ui/avatar/index.js';
  import { Textarea } from '$lib/components/ui/textarea/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { ImageIcon, GiftIcon as Gif, List, Smile, Calendar, MapPin } from '@lucide/svelte';
</script>

<div class="flex items-start gap-4 px-4 pb-4">
  <Avatar class="h-10 w-10">
    <AvatarImage src="/placeholder.svg?height=40&width=40" alt="@user" />
    <AvatarFallback>LL</AvatarFallback>
  </Avatar>
  <div class="grid flex-1 gap-2">
    <Textarea
      placeholder="What's happening?"
      class="min-h-[100px] resize-none border-none bg-transparent text-xl dark:text-white placeholder:text-[#71767b] focus-visible:ring-0
           text-black dark:placeholder:text-gray-500"
    />
    <div class="flex items-center justify-end">
      <Button
        class="rounded-full bg-[#1d9bf0] px-4 font-bold text-white hover:bg-[#1a8cd8]
           dark:bg-blue-600 dark:text-white dark:hover:bg-blue-700">Post</Button
      >
    </div>
  </div>
</div>
