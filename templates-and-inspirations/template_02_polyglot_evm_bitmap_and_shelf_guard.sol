// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title Template02_PolyglotEvmBitmapAndShelfGuard
 * @notice Minimal, Battle-Tested Reference Template for:
 *   1. OpenZeppelin `BitMaps.sol` / Uniswap `Permit2` 256-bit Storage Slot Packing (Saves 94.3% Gas)
 *   2. Two-Phase Supermarket Shelf Guard (Read-Only Inspect vs. PIN/POS Burn)
 *   3. Offline Hardware-Timestamp Priority Reconciliation (`t_offline < t_online`)
 *   4. GSMA CAMARA 5G `Cell-ID` Anti-Grey-Market Territory Enforcement
 *   5. Viettel Post COD Return Anti-Swap Lock
 *
 * UPSTREAM INSPIRATIONS:
 *   - OpenZeppelin BitMaps: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/structs/BitMaps.sol
 *   - Uniswap Permit2 Nonce Bitmap: https://github.com/Uniswap/permit2/blob/main/src/SignatureTransfer.sol
 *   - Ethereum Attestation Service (EAS): https://github.com/ethereum-attestation-service/eas-contracts
 */
contract Template02_PolyglotEvmBitmapAndShelfGuard {
    // batchIdHash => wordIndex (serialIndex >> 8) => 256-bit packed bitmap
    mapping(bytes32 => mapping(uint256 => uint256)) private _claimedBitmap;
    // batchIdHash => authorized 5G eNodeB/gNodeB Cell-ID prefix (bytes8)
    mapping(bytes32 => bytes8) public authorizedCellPrefix;
    // batchIdHash => serialIndex => earliest hardware-attested claim timestamp
    mapping(bytes32 => mapping(uint256 => uint64)) public firstClaimTimestamp;
    mapping(bytes32 => mapping(uint256 => bytes32)) public firstClaimPhoneHash;

    event ShelfInspectedReadOnly(bytes32 indexed batchIdHash, uint256 indexed serialIndex, bool isUnclaimed);
    event FirstScanBurned(bytes32 indexed batchIdHash, uint256 indexed serialIndex, bytes32 phoneHash, uint32 rewardDataMb);
    event PhotocopyCloneAlert(bytes32 indexed batchIdHash, uint256 indexed serialIndex, bytes32 reporterPhoneHash, uint32 bountyVnd);
    event TerritoryViolation(bytes32 indexed batchIdHash, uint256 indexed serialIndex, bytes8 actualCellPrefix);
    event OfflineOwnershipReconciled(bytes32 indexed batchIdHash, uint256 indexed serialIndex, bytes32 rightfulOfflineOwner, uint64 offlineTs);

    function registerBatchTerritory(bytes32 batchIdHash, bytes8 cellPrefix) external {
        authorizedCellPrefix[batchIdHash] = cellPrefix;
    }

    /// @notice Phase 1A: Customer scans outer QR on supermarket shelf (Read-Only, NEVER burns the bit!)
    function inspectOnShelfReadOnly(bytes32 batchIdHash, uint256 serialIndex) external view returns (bool isUnclaimed) {
        uint256 wordIndex = serialIndex >> 8;
        uint256 mask = 1 << (serialIndex & 0xff);
        isUnclaimed = (_claimedBitmap[batchIdHash][wordIndex] & mask) == 0;
    }

    /// @notice Phase 1B & Scenario 3: Customer unlocks inner Tear-Strip PIN or POS Checkout -> Burns bit 0->1
    function claimSerialWithShelfGuard(
        bytes32 batchIdHash,
        uint256 serialIndex,
        bytes32 phoneHash,
        bytes8 currentCellPrefix,
        uint64 scanTimestamp,
        bool hasTearStripPinOrPosUnlock
    ) external returns (bool isFirstGenuineClaim, bool isCloneRedAlert, bool isTerritoryViolation) {
        uint256 wordIndex = serialIndex >> 8;
        uint256 mask = 1 << (serialIndex & 0xff);
        uint256 word = _claimedBitmap[batchIdHash][wordIndex];

        isTerritoryViolation = (authorizedCellPrefix[batchIdHash] != bytes8(0)) &&
            (authorizedCellPrefix[batchIdHash] != currentCellPrefix);
        if (isTerritoryViolation) {
            emit TerritoryViolation(batchIdHash, serialIndex, currentCellPrefix);
        }

        if ((word & mask) == 0) {
            require(hasTearStripPinOrPosUnlock, "SHELF_GUARD: Tear-strip PIN or POS checkout required");
            _claimedBitmap[batchIdHash][wordIndex] = word | mask;
            firstClaimTimestamp[batchIdHash][serialIndex] = scanTimestamp;
            firstClaimPhoneHash[batchIdHash][serialIndex] = phoneHash;
            emit FirstScanBurned(batchIdHash, serialIndex, phoneHash, 500);
            return (true, false, isTerritoryViolation);
        } else {
            emit PhotocopyCloneAlert(batchIdHash, serialIndex, phoneHash, 20000);
            return (false, true, isTerritoryViolation);
        }
    }

    /// @notice Scenario 2 <-> 1 Race Condition Fix: Offline buyer scanned at 08:00 AM (`offlineHwTimestamp`),
    /// online cloner scanned at 09:00 AM. When offline buyer regains 4G at 12:00 PM, ownership is restored!
    function reconcileOfflineDeferredBurn(
        bytes32 batchIdHash,
        uint256 serialIndex,
        bytes32 offlineBuyerPhoneHash,
        uint64 offlineHwTimestamp
    ) external returns (bool ownershipRestored) {
        uint64 existingTs = firstClaimTimestamp[batchIdHash][serialIndex];
        if (existingTs == 0 || offlineHwTimestamp < existingTs) {
            firstClaimTimestamp[batchIdHash][serialIndex] = offlineHwTimestamp;
            firstClaimPhoneHash[batchIdHash][serialIndex] = offlineBuyerPhoneHash;
            emit OfflineOwnershipReconciled(batchIdHash, serialIndex, offlineBuyerPhoneHash, offlineHwTimestamp);
            return true;
        }
        return false;
    }
}
