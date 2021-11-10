<script lang="ts">
    import type Poll from "../models/Poll";
    import {token as tokenStore} from "../stores/token";
    import {Chart} from "chart.js";
    import {onMount} from "svelte";
    import ResultsChart from "./ResultsChart.svelte";

    let token: string | null = null;
    tokenStore.subscribe(it => {
        token = it
    })

    export let poll: Poll;

    let checked = null

    let showingResults = false;

    const onOptionCheck = (id: string) => {
        checked = id
    }

    const vote = async () => {
        console.log(checked)
        await fetch(`/api/polls/${poll.id}/vote`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({
                option: checked
            })
        })
        await showResults()
    }

    let results;
    const showResults = async () => {
        const resp = await fetch(`/api/polls/${poll.id}/results`)
        results = await resp.json().then(json => json.map((it) => {
            const option = poll.options.find(({id}) => id == it.option)
            return {
                value: it.votes,
                label: option?.text
            }
        }))
        showingResults = true
    }

    const onDialogClose = () => {
        showingResults = false
    }
</script>

<mwc-dialog heading="Pick an option" open={poll}  on:closed={onDialogClose}>
    {#if poll}
        <div class="dialog-content">
            {#if showingResults}
                <ResultsChart title={poll.title} results={results} />
            {:else}
                {#each poll.options as op}
                    <mwc-formfield label={op.text}>
                        <mwc-radio name={poll.id} on:change={() => onOptionCheck(op.id)} disabled={token === null}/>
                    </mwc-formfield>
                {/each}

                {#if token === null}
                    <span>You must be logged in to vote</span>
                {/if}

                <div class="dialog-buttons">
                    <mwc-button on:click={showResults} outlined id="results-btn">Results</mwc-button>
                    <mwc-button on:click={vote} disabled={checked === null} outlined id="vote-btn">Vote</mwc-button>
                </div>
            {/if}
        </div>
        <mwc-button dialogAction="Cancel" slot="secondaryAction">Cancel</mwc-button>
    {/if}
</mwc-dialog>

<style>
    mwc-dialog {
        --mdc-dialog-max-width: 100%;

    }
    .dialog-content {
        flex-direction: column;
    }

    .dialog-buttons {
        align-items: center;
        margin: 1rem 0;
    }

    mwc-dialog div {
        display: flex;
    }

    #vote-btn {
        margin-left: auto;
    }
</style>
