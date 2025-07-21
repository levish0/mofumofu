<script lang="ts">
  import { api } from '$lib/api/api';
  import spec from '$lib/api/spec.json';
  import { authStore } from '$lib/stores/auth.svelte';

  // --- Type Definitions ---
  type Schema = { [key: string]: any };
  type Schemas = { [key: string]: Schema };

  interface ApiEndpoint {
    id: string;
    path: string;
    method: string;
    details: any;
    pathParams: { [key: string]: string };
    queryParams: { [key: string]: string };
    body: string | null;
  }

  interface ApiResponse {
    data?: any;
    error?: any;
    status: number | null;
  }

  // --- State ---
  let apiEndpoints = $state<ApiEndpoint[]>([]);
  let responses = $state<{ [key: string]: ApiResponse }>({});
  let isLoading = $state<{ [key: string]: boolean }>({});

  // --- Effects ---
  $effect(() => {
    const endpoints: ApiEndpoint[] = [];
    for (const [path, methods] of Object.entries(spec.paths)) {
      for (const [method, d] of Object.entries(methods as any)) {
        const details: any = d;
        const endpoint: ApiEndpoint = {
          id: `${method}-${path}`,
          path,
          method: method.toUpperCase(),
          details,
          pathParams: {},
          queryParams: {},
          body: details.requestBody?.content?.['application/json']?.schema
            ? generateSampleJson(details.requestBody.content['application/json'].schema)
            : null
        };

        if (details.parameters) {
          for (const param of details.parameters as any[]) {
            if (param.in === 'path') {
              endpoint.pathParams[param.name] = '';
            } else if (param.in === 'query') {
              endpoint.queryParams[param.name] = '';
            }
          }
        }
        endpoints.push(endpoint);
      }
    }
    apiEndpoints = endpoints;
  });

  // --- Helper Functions ---
  function getReferencedSchema(ref: string): Schema | undefined {
    const path = ref.replace('#/components/schemas/', '');
    return (spec.components.schemas as Schemas)[path];
  }

  function generateSampleJson(schemaRef: { $ref: string }): string {
    if (!schemaRef || !schemaRef.$ref) return '{}';

    const schema = getReferencedSchema(schemaRef.$ref);
    if (!schema) return '{}';

    const sample: { [key: string]: any } = {};
    if (schema.properties) {
      for (const [key, p] of Object.entries(schema.properties)) {
        const prop: any = p;
        if (prop.example) {
          sample[key] = prop.example;
          continue;
        }
        switch (prop.type) {
          case 'string':
            sample[key] = prop.format === 'date-time' ? new Date().toISOString() : '';
            break;
          case 'integer':
            sample[key] = 0;
            break;
          case 'boolean':
            sample[key] = false;
            break;
          case 'array':
            sample[key] = [];
            break;
          default:
            sample[key] = {};
            break;
        }
      }
    }
    return JSON.stringify(sample, null, 2);
  }

  // --- API Call Logic ---
  async function makeRequest(endpoint: ApiEndpoint) {
    if (isLoading[endpoint.id]) return;
    isLoading[endpoint.id] = true;

    try {
      let requestPath = endpoint.path;
      for (const [key, value] of Object.entries(endpoint.pathParams)) {
        if (value === undefined || value === null || value === '') {
          responses[endpoint.id] = { error: `Path parameter "${key}" is required.`, status: 400 };
          return;
        }
        requestPath = requestPath.replace(`{${key}}`, String(value));
      }

      const searchParams = new URLSearchParams();
      for (const [key, value] of Object.entries(endpoint.queryParams)) {
        if (value) {
          searchParams.set(key, String(value));
        }
      }
      const queryString = searchParams.toString();
      if (queryString) {
        requestPath += `?${queryString}`;
      }

      const options: any = {
        method: endpoint.method,
        headers: {}
      };

      if (endpoint.body !== null) {
        try {
          options.json = JSON.parse(endpoint.body);
        } catch (e: any) {
          responses[endpoint.id] = { error: `Invalid JSON body: ${e.message}`, status: 400 };
          return;
        }
      }

      try {
        const finalRequestPath = requestPath.startsWith('/') ? requestPath.substring(1) : requestPath;
        const response = await api(finalRequestPath, options);
        let responseData: any;
        const contentType = response.headers.get('content-type');

        if (response.status === 204) {
          responseData = 'Status 204: No Content';
        } else if (contentType && contentType.includes('application/json')) {
          responseData = await response.json();
        } else {
          responseData = await response.text();
        }

        if (response.ok && responseData.access_token) {
          if (
            (endpoint.method === 'POST' && endpoint.path === '/v0/auth/login') ||
            (endpoint.method === 'POST' && endpoint.path === '/v0/auth/refresh')
          ) {
            authStore.setToken(responseData.access_token);
          }
        }

        responses[endpoint.id] = { data: responseData, status: response.status };
      } catch (error: any) {
        let errorResponse: ApiResponse = { error: 'An unknown error occurred.', status: 500 };
        if (error.response) {
          try {
            const errorData = await error.response.json();
            errorResponse = { error: errorData, status: error.response.status };
          } catch (e) {
            errorResponse = { error: await error.response.text(), status: error.response.status };
          }
        } else if (error.message) {
          errorResponse = { error: error.message, status: null };
        }
        responses[endpoint.id] = errorResponse;
      }
    } finally {
      isLoading[endpoint.id] = false;
    }
  }
</script>

<div class="font-pretendard min-h-screen bg-neutral-50 text-neutral-800 dark:bg-neutral-900 dark:text-neutral-200">
  <header
    class="sticky top-0 z-30 border-b border-neutral-200/80 bg-white/90 backdrop-blur-md dark:border-neutral-700/80 dark:bg-neutral-950/90"
  >
    <div class="container mx-auto px-5 md:px-8">
      <div class="flex h-16 items-center justify-between">
        <div class="flex items-center gap-3 select-none">
          <h1 class="text-2xl font-extrabold tracking-tight text-neutral-900 select-none dark:text-neutral-100">
            Textura API Tester
          </h1>
        </div>
      </div>
    </div>
  </header>

  <main class="container mx-auto px-5 py-8 md:px-8">
    <!-- Authentication Status -->
    <section
      class="mb-10 rounded-xl border border-neutral-200 bg-white shadow-md dark:border-neutral-700 dark:bg-neutral-800"
    >
      <div class="p-6">
        <h2 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100">Authentication Status</h2>
        {#if authStore.token}
          <div class="mt-4 space-y-4">
            <p class="text-sm font-semibold text-green-600 dark:text-green-400">✅ Authenticated</p>
            <div
              class="max-h-32 overflow-auto rounded-lg bg-neutral-100 p-3 text-xs break-words text-neutral-700 select-text dark:bg-neutral-900 dark:text-neutral-300"
            >
              {authStore.token}
            </div>
            <button
              on:click={() => authStore.clearToken()}
              class="inline-flex items-center gap-2 rounded-md bg-red-600 px-5 py-2 text-sm font-semibold text-white shadow transition hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-1 focus:outline-none dark:focus:ring-offset-neutral-900"
              aria-label="Logout and clear token"
            >
              Logout (Clear Token)
            </button>
          </div>
        {:else}
          <p class="mt-4 text-sm text-neutral-600 dark:text-neutral-400">
            🔒 Not authenticated. Use the login endpoint to get a token.
          </p>
        {/if}
      </div>
    </section>

    <!-- API Endpoints List -->
    <h2 class="mb-6 text-3xl font-bold tracking-tight text-neutral-900 dark:text-neutral-100">API Endpoints</h2>

    <div class="space-y-8">
      {#each apiEndpoints as endpoint (endpoint.id)}
        <article
          class="overflow-hidden rounded-xl border border-neutral-200 bg-white shadow-md transition-shadow hover:shadow-lg dark:border-neutral-700 dark:bg-neutral-800"
          aria-labelledby={`${endpoint.id}-title`}
        >
          <header
            class="flex items-center justify-between border-b border-neutral-200 px-6 py-4 dark:border-neutral-700"
          >
            <div class="flex items-center gap-5">
              <span
                class="flex h-11 w-11 flex-shrink-0 items-center justify-center rounded-lg text-sm font-semibold
                {endpoint.method === 'GET'
                  ? 'bg-green-100 text-green-700 dark:bg-green-900/50 dark:text-green-300'
                  : ''}
                {endpoint.method === 'POST' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300' : ''}
                {endpoint.method === 'PUT'
                  ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/50 dark:text-yellow-300'
                  : ''}
                {endpoint.method === 'DELETE' ? 'bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-300' : ''}
                "
                aria-label={`HTTP Method ${endpoint.method}`}
              >
                {endpoint.method}
              </span>
              <h3
                id={`${endpoint.id}-title`}
                class="text-lg font-semibold text-neutral-900 select-text dark:text-neutral-100"
              >
                {endpoint.path}
              </h3>
            </div>
            {#if endpoint.details.description}
              <p class="max-w-xs text-sm text-neutral-600 italic dark:text-neutral-400">
                {endpoint.details.description}
              </p>
            {/if}
          </header>

          <section class="space-y-6 p-6">
            <!-- Parameters -->
            {#if Object.keys(endpoint.pathParams).length > 0 || Object.keys(endpoint.queryParams).length > 0}
              <div>
                <h4
                  class="mb-3 border-b border-neutral-300 pb-1 text-base font-semibold text-neutral-800 dark:border-neutral-600 dark:text-neutral-200"
                >
                  Parameters
                </h4>
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
                  {#each Object.entries(endpoint.pathParams) as [paramName, value]}
                    <div>
                      <label
                        for={`${endpoint.id}-${paramName}`}
                        class="mb-1 block text-sm font-medium text-neutral-700 dark:text-neutral-300"
                      >
                        {paramName}
                        <span
                          class="ml-2 inline-block rounded-full bg-neutral-200 px-2 py-0.5 text-xs text-neutral-600 dark:bg-neutral-700 dark:text-neutral-300"
                        >
                          Path
                        </span>
                      </label>
                      <input
                        type="text"
                        id={`${endpoint.id}-${paramName}`}
                        bind:value={endpoint.pathParams[paramName]}
                        placeholder="Required"
                        class="block w-full rounded-md border border-neutral-300 bg-white px-3 py-2 text-sm shadow-sm placeholder:text-neutral-400 focus:border-blue-500 focus:ring-blue-500 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white dark:placeholder-neutral-500"
                        required
                      />
                    </div>
                  {/each}

                  {#each Object.entries(endpoint.queryParams) as [paramName, value]}
                    <div>
                      <label
                        for={`${endpoint.id}-${paramName}`}
                        class="mb-1 block text-sm font-medium text-neutral-700 dark:text-neutral-300"
                      >
                        {paramName}
                        <span
                          class="ml-2 inline-block rounded-full bg-neutral-200 px-2 py-0.5 text-xs text-neutral-600 dark:bg-neutral-700 dark:text-neutral-300"
                        >
                          Query
                        </span>
                      </label>
                      <input
                        type="text"
                        id={`${endpoint.id}-${paramName}`}
                        bind:value={endpoint.queryParams[paramName]}
                        placeholder="Optional"
                        class="block w-full rounded-md border border-neutral-300 bg-white px-3 py-2 text-sm shadow-sm placeholder:text-neutral-400 focus:border-blue-500 focus:ring-blue-500 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white dark:placeholder-neutral-500"
                      />
                    </div>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Request Body -->
            {#if endpoint.body !== null}
              <div>
                <h4
                  class="mb-3 border-b border-neutral-300 pb-1 text-base font-semibold text-neutral-800 dark:border-neutral-600 dark:text-neutral-200"
                >
                  Request Body (JSON)
                </h4>
                <textarea
                  rows="10"
                  bind:value={endpoint.body}
                  spellcheck="false"
                  class=" w-full resize-y rounded-md border border-neutral-300 bg-white px-4 py-3 text-sm leading-relaxed text-neutral-700 shadow-sm placeholder:text-neutral-400 focus:border-blue-500 focus:ring-blue-500 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white dark:placeholder-neutral-500"
                ></textarea>
              </div>
            {/if}

            <!-- Action Button -->
            <div>
              <button
                on:click={() => makeRequest(endpoint)}
                disabled={isLoading[endpoint.id]}
                class="flex w-full items-center justify-center gap-3 rounded-md bg-blue-600 px-5 py-3 text-sm font-semibold text-white shadow-md transition-colors hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 focus:outline-none disabled:cursor-not-allowed disabled:bg-blue-400 disabled:opacity-70 dark:disabled:bg-blue-800"
                aria-busy={isLoading[endpoint.id]}
                aria-disabled={isLoading[endpoint.id]}
              >
                {#if isLoading[endpoint.id]}
                  <svg
                    class="h-5 w-5 animate-spin text-white"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                  >
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                  </svg>
                  Loading...
                {:else}
                  Send Request
                {/if}
              </button>
            </div>

            <!-- Response -->
            {#if responses[endpoint.id]}
              <div>
                <h4
                  class="mb-3 border-b border-neutral-300 pb-1 text-base font-semibold text-neutral-800 dark:border-neutral-600 dark:text-neutral-200"
                >
                  Response
                </h4>
                <div
                  class="rounded-lg border px-5 py-4
                  {responses[endpoint.id].error
                    ? 'border-red-300 bg-red-50 dark:border-red-600 dark:bg-red-900/30'
                    : 'border-green-300 bg-green-50 dark:border-green-600 dark:bg-green-900/30'}"
                >
                  <div
                    class="mb-3 flex items-center justify-between border-b pb-1 text-sm font-bold
                    {responses[endpoint.id].error
                      ? 'border-red-300 text-red-700 dark:border-red-600 dark:text-red-400'
                      : 'border-green-300 text-green-700 dark:border-green-600 dark:text-green-400'}"
                  >
                    Status: {responses[endpoint.id].status ?? 'N/A'}
                  </div>
                  <pre
                    class="max-h-60 overflow-auto text-xs break-words whitespace-pre-wrap
                    {responses[endpoint.id].error
                      ? 'text-red-900 dark:text-red-200'
                      : 'text-green-900 dark:text-green-200'}">
                    {@html JSON.stringify(responses[endpoint.id].error || responses[endpoint.id].data, null, 2)}
                  </pre>
                </div>
              </div>
            {/if}
          </section>
        </article>
      {/each}
    </div>
  </main>
</div>
