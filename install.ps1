#!/usr/bin/env pwsh

# SPDX-License-Identifier: GPL-3.0-or-later OR MPL-2.0-or-later
# Copyright 2025 Nain Developers

# Install Nain

Write-Host "`e[1mWelcome to the Nain installer!`e[0m"

Write-Host ""
Write-Host "`e[36m[?]`e[0m What version of Nain do you want to install ([g]it or [r]elease)?"
$choice = Read-Host

switch -Regex ($choice) {
    "^[Gg]$" {
        Write-Host "`n`e[32m[+]`e[0m Installing Nain from GitHub..."
        if (Get-Command git -ErrorAction SilentlyContinue) {
            git clone https://github.com/nain-lang/nain.git
            Set-Location nain
            cargo install --path .
            Write-Host "`n`e[32m[+]`e[0m Nain installed successfully!"
        }
        else {
            Write-Host "`n`e[31m[!]`e[0m Git is not installed. Please install it first."
        }
    }
    "^[Rr]$" {
        Write-Host "`n`e[32m[+]`e[0m Installing Nain from crates.io..."
        cargo install nain
        if ($LASTEXITCODE -eq 0) {
            Write-Host "`n`e[32m[+]`e[0m Nain installed successfully!"
        }
        else {
            Write-Host "`n`e[31m[!]`e[0m Nain installation failed. Please try again."
            exit 1
        }
    }
    Default {
        Write-Host "`n`e[31m[!]`e[0m Invalid option. Please try again."
        exit 1
    }
}
