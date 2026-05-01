# quickstat-rop
High-performance telecom stats replay tool. Transforms 100k+ compressed XML files to current UTC ROP intervals in ~2 minutes.

## Problem Statement
In telecommunications network management, performance measurement (PM) statistics are collected and reported in 15-minute intervals known as Reporting Output Periods (ROPs). These statistics are stored as compressed XML files containing measurement data for LTE, NR, and NR_EBSN network elements. During network simulation and replay scenarios (such as radio node replay operations), there is a critical requirement to transform historical or template statistics files to match current UTC time windows while preserving the original data structure and measurement values.

The core challenge involves processing tens of thousands of compressed XML files within a tight time window (less than the 15-minute ROP interval). Each file requires decompression, timestamp manipulation (modifying beginTime and endTime attributes), and recompression, all while maintaining data integrity and handling timezone offsets present in the original file naming conventions.

Manual processing is impossible at this scale, and existing solutions suffer from performance bottlenecks that risk missing the replay window, potentially disrupting network simulation workflows.

## Core Problem Being Solved
Transform a large collection of time-stamped, compressed XML statistics files (100,000+ files) from template data to current UTC-aligned time windows efficiently enough to complete within a 15-minute operational window.
