<script lang="ts">
  import { Input } from '$lib/components/ui/input/index.js';
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Avatar, AvatarFallback, AvatarImage } from '$lib/components/ui/avatar/index.js';
  import { Search, Ellipsis } from '@lucide/svelte';

  const trendingTags = [
    { id: 1, name: '후쿠오카', posts: '1,482 posts' },
    { id: 2, name: '엔시티위시', posts: '5,389 posts' },
    { id: 3, name: '총기사고', posts: '5,114 posts' },
    { id: 4, name: '트렌드4', posts: '2,100 posts' },
    { id: 5, name: '트렌드5', posts: '999 posts' }
  ];

  const whoToFollow = [
    { id: 1, name: '藍瀬 まなみ', handle: '@AiseManami', avatar: '/placeholder.svg?height=36&width=36' },
    { id: 2, name: 'Pusu', handle: '@Pusu_kun', avatar: '/placeholder.svg?height=36&width=36' },
    { id: 3, name: 'Bendedede', handle: '@Bendedede', avatar: '/placeholder.svg?height=36&width=36' }
  ];
</script>

<div class="sticky top-0 hidden h-screen w-[350px] space-y-4 overflow-y-auto px-4 py-2 lg:block text-black dark:text-white">
  <div class="relative">
    <Search class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-[#71767b] dark:text-gray-500" />
    <Input
      placeholder="Search"
      class="rounded-full border-none bg-[#202327] pl-9 text-white placeholder:text-[#71767b] focus-visible:ring-0 focus-visible:ring-offset-0
           dark:bg-gray-100 dark:text-black dark:placeholder:text-gray-500"
    />
  </div>

<Card class="rounded-xl border-2 dark:border-zinc-800 border-gray-200 bg-transparent">
  <CardHeader>
    <CardTitle class="text-xl">Support Tuna</CardTitle>
  </CardHeader>
  <CardContent class="space-y-3">
    <p class="text-sm">
      Textuna는 광고 없이 운영됩니다. 서버 유지를 위해 도움을 주실 수 있다면 정말 감사드려요 🙏
    </p>
    <Button
      class="w-full rounded-full bg-[#1d9bf0] px-4 py-2 font-bold text-white hover:bg-[#1a8cd8]
        dark:bg-blue-600 dark:hover:bg-blue-700"
    >
      Subscribe
    </Button>
    <a
      href="https://www.patreon.com/yourpatreonid"  
      target="_blank"
      rel="noopener noreferrer"
    >
      <Button
      class="w-full rounded-full bg-[#1d9bf0] px-4 py-2 font-bold text-white hover:bg-[#1a8cd8]
        dark:bg-blue-600 dark:hover:bg-blue-700"
    >
      Patreon
    </Button>
  </a>
  </CardContent>
</Card>


  <Card class="rounded-xl border-2 dark:border-zinc-800 border-gray-200 bg-transparent ">
    <CardHeader class="pb-2">
      <CardTitle class="text-xl ">What's happening</CardTitle>
    </CardHeader>
    <CardContent class="space-y-3">
      {#each trendingTags as tag}
        <div class="group flex items-center justify-between">
          <div class="grid gap-0.5">
            <span class="text-xs ">Trending in South Korea</span>
            <a href="#" class="font-bold hover:underline ">{tag.name}</a>
            <span class="text-xs text-[#71767b] dark:text-gray-500">{tag.posts}</span>
          </div>
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 rounded-full text-[#71767b] opacity-0 transition-opacity group-hover:opacity-100 hover:bg-blue-900/20 hover:text-[#1d9bf0]
           dark:text-gray-500 dark:hover:bg-blue-100/20 dark:hover:text-blue-700"
          >
            <Ellipsis class="h-4 w-4" /> <span class="sr-only">More options</span>
          </Button>
        </div>
      {/each} <a href="#" class="text-sm text-[#1d9bf0] hover:underline dark:text-blue-700">Show more</a>
    </CardContent>
  </Card>

  <Card class="rounded-xl border-2 dark:border-zinc-800 border-gray-200 bg-transparent ">
    <CardHeader class="pb-2">
      <CardTitle class="text-xl text-black dark:text-white">Who to follow</CardTitle>
    </CardHeader>
    <CardContent class="space-y-3">
      {#each whoToFollow as user}
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <Avatar class="h-9 w-9">
              <AvatarImage src={user.avatar} alt={user.name} />
              <AvatarFallback>{user.name[0]}</AvatarFallback>
            </Avatar>
            <div class="grid leading-none">
              <div class="font-bold text-white dark:text-black">{user.name}</div>
              <div class="text-sm text-[#71767b] dark:text-gray-500">{user.handle}</div>
            </div>
          </div>
          <Button
            class="rounded-full bg-white px-4 py-2 font-bold text-black hover:bg-gray-200
           dark:bg-black dark:text-white dark:hover:bg-gray-800">Follow</Button
          >
        </div>
      {/each} <a href="#" class="text-sm text-[#1d9bf0] hover:underline dark:text-blue-700">Show more</a>
    </CardContent>
  </Card>
</div>
