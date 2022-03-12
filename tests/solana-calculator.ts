import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { SolanaCalculator } from "../target/types/solana_calculator"
import assert from "assert"
const { SystemProgram } = anchor.web3

describe("solana-calculator", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.Provider.local()
	anchor.setProvider(provider)
	const calculator = anchor.web3.Keypair.generate()
	const program = anchor.workspace.SolanaCalculator as Program<SolanaCalculator>
	let _calculator: anchor.web3.Keypair
	it("Creates a calculator", async () => {
		await program.rpc.create("Welcome to solana", {
			accounts: {
				calculator: calculator.publicKey,
				user: provider.wallet.publicKey,
				systemProgram: SystemProgram.programId,
			},
			signers: [calculator],
		})

		const account = await program.account.calculator.fetch(calculator.publicKey)
		assert.ok(account.greeting === "Welcome to solana")
		_calculator = calculator
	})

	it("Adds two numbers", async function () {
		const calculator = _calculator
		await program.rpc.addition(new anchor.BN(2), new anchor.BN(3), {
			accounts: {
				calculator: calculator.publicKey,
			},
		})

		const account = await program.account.calculator.fetch(calculator.publicKey)
		assert.ok(account.result.eq(new anchor.BN(5)))
		assert.ok(account.greeting === "Welcome to solana")
	})

	it("Multiplies two numbers", async function () {
		const calculator = _calculator
		await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
			accounts: {
				calculator: calculator.publicKey,
			},
		})

		const account = await program.account.calculator.fetch(calculator.publicKey)
		assert.ok(account.result.eq(new anchor.BN(6)))
		assert.ok(account.greeting === "Welcome to solana")
	})

	it("Subtracts two numbers", async function () {
		const calculator = _calculator
		await program.rpc.subtract(new anchor.BN(5), new anchor.BN(4), {
			accounts: {
				calculator: calculator.publicKey,
			},
		})
		const account = await program.account.calculator.fetch(calculator.publicKey)
		assert.ok(account.result.eq(new anchor.BN(1)))
		assert.ok(account.greeting === "Welcome to solana")
	})

	it("Divides two numbers", async function () {
		const calculator = _calculator
		await program.rpc.divide(new anchor.BN(10), new anchor.BN(5), {
			accounts: {
				calculator: calculator.publicKey,
			},
		})

		const account = await program.account.calculator.fetch(calculator.publicKey)
		assert.ok(account.result.eq(new anchor.BN(2)))
		assert.ok(account.remainder.eq(new anchor.BN(0)))
		assert.ok(account.greeting === "Welcome to solana")
	})
})
