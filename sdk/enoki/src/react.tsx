// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useStore } from '@nanostores/react';
import type { ReactNode } from 'react';
import { createContext, useContext, useState } from 'react';

import type { EnokiFlowConfig } from './EnokiFlow.js';
import { EnokiFlow } from './EnokiFlow.js';

const EnokiFlowContext = createContext<EnokiFlow | null>(null);

export interface EnokiFlowProviderProps extends EnokiFlowConfig {
	children: ReactNode;
}

export function EnokiFlowProvider({ children, ...config }: EnokiFlowProviderProps) {
	const [enokiFlow] = useState(() => new EnokiFlow(config));
	return <EnokiFlowContext.Provider value={enokiFlow}>{children}</EnokiFlowContext.Provider>;
}

// TODO: Should this just subscribe to the store too?
export function useEnokiFlow() {
	const context = useContext(EnokiFlowContext);
	if (!context) {
		throw new Error('Missing `EnokiFlowContext` provider');
	}
	return context;
}

export function useEnokiFlowState() {
	const flow = useEnokiFlow();
	const state = useStore(flow.$state);
	const initialized = useStore(flow.$initialized);
	return { ...state, initialized };
}
