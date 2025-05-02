#!/bin/bash

# SPDX-License-Identifier: GPL-3.0-or-later OR MPL-2.0-or-later
# Copyright 2025 Nain Developers

# Install Nain

echo -e "\x1b[1mWelcome to the Nain installer!\x1b[0m"

echo -e "\n\x1b[36m[?]\x1b[0m What version of Nain do you want to install ([g]it or [r]elease)?"

read -p "" -n 1 -r

if [[ $REPLY =~ ^[Gg]$ ]]; then
	echo -e "\n\x1b[32m[+]\x1b[0m Installing Nain from GitHub..."
	if command -v git >/dev/null 2>&1; then
		git clone https://github.com/nain-lang/nain.git
		cd nain
		cargo install --path .
		echo -e "\n\x1b[32m[+]\x1b[0m Nain installed successfully!"
	else
		echo -e "\n\x1b[31m[!]\x1b[0m Git is not installed. Please install it first."
	fi
elif [[ $REPLY =~ ^[Rr]$ ]]; then
	echo -e "\n\x1b[32m[+]\x1b[0m Installing Nain from crates.io..."
	cargo install nain
	if [[ $? -eq 0 ]]; then
		echo -e "\n\x1b[32m[+]\x1b[0m Nain installed successfully!"
	else
		echo -e "\n\x1b[31m[!]\x1b[0m Nain installation failed. Please try again."
		exit 1
	fi
else
	echo -e "\n\x1b[31m[!]\x1b[0m Invalid option. Please try again."
	exit 1
fi
