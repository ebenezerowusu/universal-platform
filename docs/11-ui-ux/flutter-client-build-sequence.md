# Flutter Client Build Sequence

## Purpose

This document defines the recommended build order for the Flutter client.

The goal is to create a shared client foundation before adding feature-heavy screens.

## Build Principle

Build the shell before the features.

Shared state, routing, design system, and API handling should come before domain screens.

## Step 1: Client Structure

Create the first Flutter structure for:

- app startup
- shared UI
- client services
- feature folders
- routing
- layouts

## Step 2: Shared UI Components

Create reusable components for:

- buttons
- text fields
- cards
- lists
- page headers
- loading state
- empty state
- status labels

## Step 3: API Client

Create the first API client wrapper for:

- base URL setup
- standard response parsing
- standard error parsing
- request metadata

## Step 4: Shared App State

Create state areas for:

- user summary
- selected organization
- language
- module summary
- feature summary
- navigation summary

## Step 5: Route Groups

Create route groups for:

- start flow
- organization selection
- mobile shell
- desktop shell
- feature areas

## Step 6: First Screens

Create:

- login screen
- organization selector
- mobile home shell
- desktop dashboard shell

## Step 7: First Religious Screens

After backend contracts are stable, create:

- member list
- member detail
- visitor list
- congregation list
- service list
- group list
- attendance list

## Step 8: Tests

Add tests for:

- app startup
- API response parsing
- login screen
- organization selector
- mobile shell
- desktop shell
- shared components

## Final Rule

The Flutter client should grow from shared foundation into modules, not from disconnected pages.
