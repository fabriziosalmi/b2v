---
layout: default
title: Architecture Overview
---

# Architecture Overview

This guide explains the system architecture of b2v, including key components and their interactions.

## Key Components

- **Decoder**: Handles input data parsing and conversion to internal representations.
- **Encoder**: Manages output formatting and serialization of processed data.
- **Utils**: Provides utility functions for common operations across the system.

## Component Interaction

The decoder and encoder work in tandem, with utils supporting both through helper functions. This design ensures modularity and ease of maintenance.
