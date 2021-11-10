<script lang="ts">
    import Auth from './components/AuthDialog.svelte'
    import PollCard from "./components/PollCard.svelte";
    import PollDialog from "./components/PollDialog.svelte";

    import {token, token as tokenStore} from './stores/token'
    import {onMount} from "svelte";
    import type Poll from "./models/Poll";
    import CreatePoll from "./components/CreatePoll.svelte";

    let polls: Poll[] = []

    let currentToken;
    tokenStore.subscribe(tokenChange => {
        currentToken = tokenChange
    })

    let loginDialog;

    const showLoginDialog = () => {
        loginDialog.show()
    }

    const refreshToken = async (refresh: string) => {
        const res = await fetch("/api/auth/refresh", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({refresh})
        })
        if (res.status !== 200) {
            return
        }
        const json = await res.json()
        token.set(json.token)
        const exp = new Date(json.duration)
        const duration = exp - Date.now()
        setTimeout(() => {
            refreshToken(json.refresh)
        }, duration)
    }

    const onSignIn = ({token, exp, refresh}: { token: string, refresh: string, exp: Date }) => {
        localStorage.setItem('refresh', refresh)
        tokenStore.set(token)
        const duration = exp - Date.now()
        console.log('duration', duration)
        loginDialog.close()
        setTimeout(() => {
            refreshToken(refresh)
        }, duration)
    }

    onMount(async () => {
        const resp = await fetch('/api/polls')
        polls = await resp.json<Poll>()
        const refresh = localStorage.getItem('refresh')
        if (refresh) {
            await refreshToken(refresh)
        }
    })

    let selectedPoll: Poll | null = null

    const onPollSelect = (selected: CustomEvent<Poll>) => {
        console.log({selected})
        selectedPoll = selected.detail
    }

    const pollCreate = (poll: CustomEvent<Poll>) => {
        polls = [poll.detail, ...polls]
        console.log({polls})
    }
</script>

<mwc-top-app-bar>
    <div slot="title">Electer</div>
    {#if currentToken !== null}
        <mwc-icon-button icon="person" slot="actionItems"/>
    {:else}
        <mwc-icon-button icon="login" slot="actionItems" on:click={showLoginDialog}/>
    {/if}
</mwc-top-app-bar>

<main>
    <h1>Select polls to vote on</h1>

    <section class="polls">
        {#each polls as poll}
            <PollCard on:click={onPollSelect} poll={poll} />
        {/each}
    </section>
</main>

<mwc-dialog bind:this={loginDialog} id="auth-dialog">
    <Auth on:signedIn={(e) => onSignIn(e.detail)}/>
</mwc-dialog>

<PollDialog poll={selectedPoll}/>

<CreatePoll on:pollCreate={pollCreate} />

<style>
    main {
        padding: 2rem;
    }

    main h1 {
        padding-bottom: 2rem;
    }

    .polls {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(24rem, 1fr));
        grid-column-gap: 1rem;
        grid-row-gap: 2rem;
        justify-items: center;
    }

    #auth-dialog {
        --mdc-dialog-min-width: 300px;
    }
</style>
