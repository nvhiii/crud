'use client'

import { getCrudapp2Program, getCrudapp2ProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

interface CreateEntryArgs {
  title: string;
  message: string;
  owner: PublicKey;
}

export function useCrudapp2Program() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getCrudapp2ProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getCrudapp2Program(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['crudapp2', 'all', { cluster }],
    queryFn: () => program.account.journalEntryState.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const createEntry = useMutation<string, Error, CreateEntryArgs>({

    mutationKey: ['journalEntry', 'create', { cluster }],
    mutationFn: async ({title, message, owner}) => {

      return program.methods.createJournalEntry(title, message).rpc();

    },
    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },

    // on error
    onError: (error) => {

      toast.error(`Error creating entry! ${error.message}`)
  
    }

  });

  return {
    program,
    accounts,
    getProgramAccount,
    createEntry
  };

}

export function useCrudapp2ProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useCrudapp2Program()

  const accountQuery = useQuery({
    queryKey: ['crudapp2', 'fetch', { cluster, account }],
    queryFn: () => program.account.journalEntryState.fetch(account),
  })

  const createEntry = useMutation<string, Error, CreateEntryArgs>

  return {
    accountQuery,

  }
}
