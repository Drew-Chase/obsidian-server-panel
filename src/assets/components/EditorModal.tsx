import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader} from "@nextui-org/react";
import {Editor} from "@monaco-editor/react";
import {toast} from "sonner";
import "../scss/editor.scss";

interface EditorModalProps
{
    isOpen: boolean;
    onClose: () => void;
}

export default function EditorModal(props: EditorModalProps)
{
    return (
        <Modal
            isOpen={props.isOpen}
            onClose={props.onClose}
            size={"5xl"}

        >
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>Code Editor</ModalHeader>
                        <ModalBody>
                            <Editor
                                height="70vh"
                                defaultLanguage="log"
                                defaultValue={crashReport.trim()}
                                theme={"vs-dark"}
                            />
                        </ModalBody>
                        <ModalFooter>
                            <Button onClick={() =>
                            {
                                toast("Changes saved successfully!", {description: "The crash report has been saved successfully."});
                                onClose();
                            }} color={"primary"}>Save</Button>
                            <Button onClick={onClose} color={"danger"} variant={"light"}>Cancel</Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>

        </Modal>
    );
}


const crashReport = `
---- Minecraft Crash Report ----
// Ooh. Shiny.

Time: 2024-09-10 01:26:07
Description: Ticking entity

java.util.NoSuchElementException: No value present
\tat java.base/java.util.Optional.get(Optional.java:143)
\tat net.minecraft.class_4810.method_24568(class_4810.java:107)
\tat net.minecraft.class_4810.method_24570(class_4810.java:42)
\tat net.minecraft.class_4810.method_18919(class_4810.java:20)
\tat net.minecraft.class_4097.method_18922(class_4097.java:47)
\tat net.minecraft.class_4095.method_18891(class_4095.java:736)
\tat net.minecraft.class_4095.method_19542(class_4095.java:492)
\tat net.minecraft.class_4836.method_5958(class_4836.java:330)
\tat net.minecraft.class_1308.method_6023(class_1308.java:792)
\tat net.minecraft.class_1309.method_6007(class_1309.java:2697)
\tat net.minecraft.class_1308.method_6007(class_1308.java:556)
\tat net.minecraft.class_1588.method_6007(class_1588.java:44)
\tat net.minecraft.class_1309.method_5773(class_1309.java:2446)
\tat net.minecraft.class_1308.method_5773(class_1308.java:357)
\tat net.minecraft.class_3218.method_18762(class_3218.java:761)
\tat net.minecraft.class_1937.method_18472(class_1937.java:492)
\tat net.minecraft.class_3218.method_31420(class_3218.java:399)
\tat net.minecraft.class_5574.method_31791(class_5574.java:54)
\tat net.minecraft.class_3218.method_18765(class_3218.java:363)
\tat net.minecraft.server.MinecraftServer.mixinextras$bridge$method_18765$292(MinecraftServer.java)
\tat net.minecraft.server.MinecraftServer.wrapOperation$cdg000$carpet-tis-addition$yeetUpdateSuppressionCrash_implOnTickWorlds(MinecraftServer.java:6413)
\tat net.minecraft.server.MinecraftServer.method_3813(MinecraftServer.java:948)
\tat net.minecraft.class_3176.method_3813(class_3176.java:283)
\tat net.minecraft.server.MinecraftServer.method_3748(MinecraftServer.java:845)
\tat net.minecraft.server.MinecraftServer.method_29741(MinecraftServer.java:683)
\tat net.minecraft.server.MinecraftServer.method_29739(MinecraftServer.java:270)
\tat java.base/java.lang.Thread.run(Thread.java:833)


A detailed walkthrough of the error, its code path and all known details is as follows:
---------------------------------------------------------------------------------------

-- Head --
Thread: Server thread
Stacktrace:
\tat java.base/java.util.Optional.get(Optional.java:143)
\tat net.minecraft.class_4810.method_24568(class_4810.java:107)
\tat net.minecraft.class_4810.method_24570(class_4810.java:42)
\tat net.minecraft.class_4810.method_18919(class_4810.java:20)
\tat net.minecraft.class_4097.method_18922(class_4097.java:47)
\tat net.minecraft.class_4095.method_18891(class_4095.java:736)
\tat net.minecraft.class_4095.method_19542(class_4095.java:492)
\tat net.minecraft.class_4836.method_5958(class_4836.java:330)
\tat net.minecraft.class_1308.method_6023(class_1308.java:792)
\tat net.minecraft.class_1309.method_6007(class_1309.java:2697)
\tat net.minecraft.class_1308.method_6007(class_1308.java:556)
\tat net.minecraft.class_1588.method_6007(class_1588.java:44)
\tat net.minecraft.class_1309.method_5773(class_1309.java:2446)
\tat net.minecraft.class_1308.method_5773(class_1308.java:357)
\tat net.minecraft.class_3218.method_18762(class_3218.java:761)
\tat net.minecraft.class_1937.method_18472(class_1937.java:492)
\tat net.minecraft.class_3218.method_31420(class_3218.java:399)
\tat net.minecraft.class_5574.method_31791(class_5574.java:54)
\tat net.minecraft.class_3218.method_18765(class_3218.java:363)
\tat net.minecraft.server.MinecraftServer.mixinextras$bridge$method_18765$292(MinecraftServer.java)
Mixins in Stacktrace:
\tnet.minecraft.class_4097:
\t\tme.jellysquid.mods.lithium.mixin.ai.task.memory_change_counting.MultiTickTaskMixin (lithium.mixins.json)
\tnet.minecraft.class_4095:
\t\tme.jellysquid.mods.lithium.mixin.ai.task.launch.BrainMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.collections.brain.BrainMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.ai.task.memory_change_counting.BrainMixin (lithium.mixins.json)
\tnet.minecraft.class_4836:
\t\tcarpettisaddition.mixins.rule.spawnBabyProbably.PiglinEntityMixin (carpet-tis-addition.mixins.json)
\tnet.minecraft.class_1308:
\t\tcarpettisaddition.mixins.rule.keepMobInLazyChunks.MobEntityMixin (carpet-tis-addition.mixins.json)
\t\tcom.ishland.vmp.mixins.playerwatching.optimize_nearby_player_lookups.MixinMobEntity (vmp.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.entity.MobEntityAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.skip_equipment_change_check.MobEntityMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.removal.conversion.MobEntityMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.inactive_navigations.MobEntityMixin (lithium.mixins.json)
\t\tcarpet.mixins.MobMixin (carpet.mixins.json)
\t\tnet.fabricmc.fabric.mixin.entity.event.MobEntityMixin (fabric-entity-events-v1.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.removal.mobpickup.MobEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.spawning.conversion.MobEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.removal.despawn.MobEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.removal.persistent.MobEntityMixin (carpet-tis-addition.mixins.json)
\tnet.minecraft.class_1309:
\t\tcarpettisaddition.mixins.command.lifetime.spawning.vehicledismounting.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.voidDamageIgnorePlayer.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.fast_powder_snow_check.LivingEntityMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.enum_values.living_entity.LivingEntityMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.damage.LivingEntityAndPlayerEntityMixins$DamageMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.item.LivingEntityMixin (fabric-item-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.spawning.mobdrop.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.fast_hand_swing.LivingEntityMixin (lithium.mixins.json)
\t\tcarpet.mixins.LivingEntity_scarpetEventsMixin (carpet.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinLivingEntity (architectury.mixins.json)
\t\tcarpet.mixins.LivingEntity_cleanLogsMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.deathdamage.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.LivingEntity_creativeFlyMixin (carpet.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.fast_elytra_check.LivingEntityMixin (lithium.mixins.json)
\t\tcarpet.mixins.LivingEntity_maxCollisionsMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.damage.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.entityInstantDeathRemoval.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.damage.LivingEntityAndPlayerEntityMixins$ApplyDamageMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.LivingEntityMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.skip_equipment_change_check.LivingEntityMixin (lithium.mixins.json)
\t\tdev.architectury.mixin.fabric.LivingDeathInvoker (architectury.mixins.json)
\t\tcarpettisaddition.mixins.rule.voidDamageAmount.LivingEntityMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.collisions.unpushable_cramming.LivingEntityMixin (lithium.mixins.json)
\t\tcarpetextra.mixins.LivingEntityMixin (carpet-extra.mixins.json)
\t\tnet.fabricmc.fabric.mixin.entity.event.LivingEntityMixin (fabric-entity-events-v1.mixins.json)
\t\tnet.fabricmc.fabric.mixin.entity.event.elytra.LivingEntityMixin (fabric-entity-events-v1.mixins.json)
\tnet.minecraft.class_3218:
\t\tcarpettisaddition.mixins.command.info.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.ServerLevel_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateExceptMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.attachment.ServerWorldMixin (fabric-data-attachment-api-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.profiler.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.block_entity_ticking.sleeping.ServerWorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.accessors.ServerWorldAccessor (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.chunk_random.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.inactive_navigations.ServerWorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.ServerLevel_spawnChunksMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.synchronizedLightThread.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.chunkTickSpeed.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.cache_strongholds.ServerLevelMixin (modernfix-common.mixins.json)
\t\tcom.ishland.vmp.mixins.playerwatching.optimize_nearby_player_lookups.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.obsidianPlatformBlockBreakerBackport.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.entity.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.poiUpdates.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.blockEventPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinServerLevel (architectury.mixins.json)
\t\tcarpettisaddition.mixins.rule.explosionPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.ServerLevelMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.bugfix.chunk_deadlock.ServerLevelMixin (modernfix-common.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.ServerWorldMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tcom.ishland.vmp.mixins.chunk.ticking.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.tileTickLimit.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.lucko.spark.fabric.mixin.ServerWorldAccessor (spark.mixins.json)
\t\tcarpet.mixins.ServerLevel_tickMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdate2Mixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.hooks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.chunktick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.tiletick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.worldborder.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.container.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.entity_movement_tracking.ServerWorldAccessor (lithium.mixins.json)
\t\tnet.fabricmc.fabric.mixin.lookup.ServerWorldMixin (fabric-api-lookup-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.rule.keepMobInLazyChunks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\tnet.minecraft.class_1937:
\t\tnet.fabricmc.fabric.mixin.attachment.AttachmentTargetsMixin (fabric-data-attachment-api-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.chunk_access.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_getOtherEntitiesLimited (carpet.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.collisions.intersection.WorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.block_entity_ticking.sleeping.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.rule.instantBlockUpdaterReintroduced.WorldMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.WorldMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.inline_block_access.WorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.block_entity_retrieval.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_movableBEMixin (carpet.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.block_tracking.block_listening.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.tileentity.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.container.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinLevel (architectury.mixins.json)
\t\tcarpettisaddition.mixins.carpet.tweaks.rule.tntRandomRange.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.chunk_random.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleStateUpdateMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.inline_height.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_tickMixin (carpet.mixins.json)
\t\tcarpet.mixins.Level_fillUpdatesMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.entity.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockstatechange.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$ComparatorUpdateMixin (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.Level_scarpetPlopMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.utils.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.block.hopper.WorldMixin (lithium.mixins.json)
\tnet.minecraft.class_5574:
\t\tcarpettisaddition.mixins.command.manipulate.container.EntityListAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.collections.entity_ticking.EntityListMixin (lithium.mixins.json)
\tnet.minecraft.server.MinecraftServer:
\t\tnet.fabricmc.fabric.mixin.registry.sync.MinecraftServerMixin (fabric-registry-sync-v0.mixins.json)
\t\torg.embeddedt.modernfix.fabric.mixin.core.MinecraftServerMixin (modernfix-fabric.mixins.json)
\t\tcarpet.mixins.MinecraftServer_pingPlayerSampleLimit (carpet.mixins.json)
\t\tcarpet.mixins.MinecraftServer_coreMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.syncServerMsptMetricsData.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tch.endte.syncmatica.mixin.MixinMinecraftServer (syncmatica.mixin.json)
\t\tcarpet.mixins.MinecraftServer_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.asynctask.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.MinecraftServerMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.dedicated_reload_executor.MinecraftServerMixin (modernfix-common.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.api.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.biome.modification.MinecraftServerMixin (fabric-biome-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.resource.loader.MinecraftServerMixin (fabric-resource-loader-v0.mixins.json)
\t\tcarpet.mixins.MinecraftServer_tickspeedMixin (carpet.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.MinecraftServerMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tfi.dy.masa.servux.mixin.MixinMinecraftServer (mixins.servux.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.autosave.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.message.MinecraftServerMixin (fabric-message-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.carpet.hooks.onServerLoadedWorlds.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.yeetUpdateSuppressionCrash.yeet.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcom.sk89q.worldedit.fabric.mixin.MixinMinecraftServer (worldedit-fabric.mixins.json)

-- Entity being ticked --
Details:
\tEntity Type: minecraft:piglin (net.minecraft.class_4836)
\tEntity ID: 5831
\tEntity Name: Piglin
\tEntity's Exact location: 240.01, 95.00, 1502.00
\tEntity's Block location: World: (240,95,1502), Section: (at 0,15,14 in 15,5,93; chunk contains blocks 240,0,1488 to 255,255,1503), Region: (0,2; contains chunks 0,64 to 31,95, blocks 0,0,1024 to 511,255,1535)
\tEntity's Momentum: 0.10, -0.08, -0.10
\tEntity's Passengers: []
\tEntity's Vehicle: null
Stacktrace:
\tat net.minecraft.class_1937.method_18472(class_1937.java:492)
\tat net.minecraft.class_3218.method_31420(class_3218.java:399)
\tat net.minecraft.class_5574.method_31791(class_5574.java:54)
\tat net.minecraft.class_3218.method_18765(class_3218.java:363)
\tat net.minecraft.server.MinecraftServer.mixinextras$bridge$method_18765$292(MinecraftServer.java)
\tat net.minecraft.server.MinecraftServer.wrapOperation$cdg000$carpet-tis-addition$yeetUpdateSuppressionCrash_implOnTickWorlds(MinecraftServer.java:6413)
\tat net.minecraft.server.MinecraftServer.method_3813(MinecraftServer.java:948)
\tat net.minecraft.class_3176.method_3813(class_3176.java:283)
\tat net.minecraft.server.MinecraftServer.method_3748(MinecraftServer.java:845)
\tat net.minecraft.server.MinecraftServer.method_29741(MinecraftServer.java:683)
\tat net.minecraft.server.MinecraftServer.method_29739(MinecraftServer.java:270)
\tat java.base/java.lang.Thread.run(Thread.java:833)
Mixins in Stacktrace:
\tnet.minecraft.class_1937:
\t\tnet.fabricmc.fabric.mixin.attachment.AttachmentTargetsMixin (fabric-data-attachment-api-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.chunk_access.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_getOtherEntitiesLimited (carpet.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.collisions.intersection.WorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.block_entity_ticking.sleeping.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.rule.instantBlockUpdaterReintroduced.WorldMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.WorldMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.inline_block_access.WorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.block_entity_retrieval.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_movableBEMixin (carpet.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.block_tracking.block_listening.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.tileentity.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.container.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinLevel (architectury.mixins.json)
\t\tcarpettisaddition.mixins.carpet.tweaks.rule.tntRandomRange.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.chunk_random.WorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleStateUpdateMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.inline_height.WorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.Level_tickMixin (carpet.mixins.json)
\t\tcarpet.mixins.Level_fillUpdatesMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.entity.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockstatechange.WorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$ComparatorUpdateMixin (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.Level_scarpetPlopMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.utils.WorldAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.block.hopper.WorldMixin (lithium.mixins.json)
\tnet.minecraft.class_3218:
\t\tcarpettisaddition.mixins.command.info.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.ServerLevel_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateExceptMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.attachment.ServerWorldMixin (fabric-data-attachment-api-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.profiler.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.block_entity_ticking.sleeping.ServerWorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.accessors.ServerWorldAccessor (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.chunk_random.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.inactive_navigations.ServerWorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.ServerLevel_spawnChunksMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.synchronizedLightThread.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.chunkTickSpeed.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.cache_strongholds.ServerLevelMixin (modernfix-common.mixins.json)
\t\tcom.ishland.vmp.mixins.playerwatching.optimize_nearby_player_lookups.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.obsidianPlatformBlockBreakerBackport.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.entity.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.poiUpdates.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.blockEventPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinServerLevel (architectury.mixins.json)
\t\tcarpettisaddition.mixins.rule.explosionPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.ServerLevelMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.bugfix.chunk_deadlock.ServerLevelMixin (modernfix-common.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.ServerWorldMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tcom.ishland.vmp.mixins.chunk.ticking.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.tileTickLimit.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.lucko.spark.fabric.mixin.ServerWorldAccessor (spark.mixins.json)
\t\tcarpet.mixins.ServerLevel_tickMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdate2Mixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.hooks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.chunktick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.tiletick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.worldborder.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.container.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.entity_movement_tracking.ServerWorldAccessor (lithium.mixins.json)
\t\tnet.fabricmc.fabric.mixin.lookup.ServerWorldMixin (fabric-api-lookup-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.rule.keepMobInLazyChunks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\tnet.minecraft.class_5574:
\t\tcarpettisaddition.mixins.command.manipulate.container.EntityListAccessor (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.collections.entity_ticking.EntityListMixin (lithium.mixins.json)
\tnet.minecraft.server.MinecraftServer:
\t\tnet.fabricmc.fabric.mixin.registry.sync.MinecraftServerMixin (fabric-registry-sync-v0.mixins.json)
\t\torg.embeddedt.modernfix.fabric.mixin.core.MinecraftServerMixin (modernfix-fabric.mixins.json)
\t\tcarpet.mixins.MinecraftServer_pingPlayerSampleLimit (carpet.mixins.json)
\t\tcarpet.mixins.MinecraftServer_coreMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.syncServerMsptMetricsData.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tch.endte.syncmatica.mixin.MixinMinecraftServer (syncmatica.mixin.json)
\t\tcarpet.mixins.MinecraftServer_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.asynctask.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.MinecraftServerMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.dedicated_reload_executor.MinecraftServerMixin (modernfix-common.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.api.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.biome.modification.MinecraftServerMixin (fabric-biome-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.resource.loader.MinecraftServerMixin (fabric-resource-loader-v0.mixins.json)
\t\tcarpet.mixins.MinecraftServer_tickspeedMixin (carpet.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.MinecraftServerMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tfi.dy.masa.servux.mixin.MixinMinecraftServer (mixins.servux.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.autosave.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.message.MinecraftServerMixin (fabric-message-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.carpet.hooks.onServerLoadedWorlds.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.yeetUpdateSuppressionCrash.yeet.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcom.sk89q.worldedit.fabric.mixin.MixinMinecraftServer (worldedit-fabric.mixins.json)
\tnet.minecraft.class_3176:
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.console.MinecraftDedicatedServerMixin (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinDedicatedServer (architectury.mixins.json)

-- Affected level --
Details:
\tAll players: 1 total; [class_3222['dcman58'/655, l='ServerLevel[world]', x=184.06, y=163.53, z=1408.97]]
\tChunk stats: 3362
\tLevel dimension: minecraft:the_nether
\tDerived: true
\tLevel spawn location: World: (0,83,0), Section: (at 0,3,0 in 0,5,0; chunk contains blocks 0,0,0 to 15,255,15), Region: (0,0; contains chunks 0,0 to 31,31, blocks 0,0,0 to 511,255,511)
\tLevel time: 685099305 game time, 693575244 day time
\tLevel name: world
\tLevel game mode: Game mode: survival (ID 0). Hardcore: false. Cheats: false
\tLevel weather: Rain time: 21784 (now: true), thunder time: 122792 (now: false)
\tKnown server brands: fabric
\tRemoved feature flags: 
\tLevel was modded: true
\tLevel storage version: 0x04ABD - Anvil
\tLoaded entity count: 250
Stacktrace:
\tat net.minecraft.class_3218.method_8538(class_3218.java:1681)
\tat net.minecraft.server.MinecraftServer.method_3813(MinecraftServer.java:951)
\tat net.minecraft.class_3176.method_3813(class_3176.java:283)
\tat net.minecraft.server.MinecraftServer.method_3748(MinecraftServer.java:845)
\tat net.minecraft.server.MinecraftServer.method_29741(MinecraftServer.java:683)
\tat net.minecraft.server.MinecraftServer.method_29739(MinecraftServer.java:270)
\tat java.base/java.lang.Thread.run(Thread.java:833)
Mixins in Stacktrace:
\tnet.minecraft.class_3218:
\t\tcarpettisaddition.mixins.command.info.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpet.mixins.ServerLevel_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateExceptMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$BlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.attachment.ServerWorldMixin (fabric-data-attachment-api-v1.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.profiler.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.world.block_entity_ticking.sleeping.ServerWorldMixin (lithium.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.accessors.ServerWorldAccessor (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.alloc.chunk_random.ServerWorldMixin (lithium.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.entity.inactive_navigations.ServerWorldMixin (lithium.mixins.json)
\t\tcarpet.mixins.ServerLevel_spawnChunksMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.synchronizedLightThread.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.chunkTickSpeed.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.cache_strongholds.ServerLevelMixin (modernfix-common.mixins.json)
\t\tcom.ishland.vmp.mixins.playerwatching.optimize_nearby_player_lookups.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.obsidianPlatformBlockBreakerBackport.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.entity.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.lifetime.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.poiUpdates.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.blockEventPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinServerLevel (architectury.mixins.json)
\t\tcarpettisaddition.mixins.rule.explosionPacketRange.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.ServerLevelMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.bugfix.chunk_deadlock.ServerLevelMixin (modernfix-common.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.ServerWorldMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tcom.ishland.vmp.mixins.chunk.ticking.MixinServerWorld (vmp.mixins.json)
\t\tcarpettisaddition.mixins.rule.tileTickLimit.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.lucko.spark.fabric.mixin.ServerWorldAccessor (spark.mixins.json)
\t\tcarpet.mixins.ServerLevel_tickMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdate2Mixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.hooks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.chunktick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.tiletick.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.worldborder.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.events.blockupdate.WorldMixins$SingleBlockUpdateMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.command.manipulate.container.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.ServerWorldAccessor (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.blockevent.ServerWorldMixin (carpet-tis-addition.mixins.json)
\t\tme.jellysquid.mods.lithium.mixin.util.entity_movement_tracking.ServerWorldAccessor (lithium.mixins.json)
\t\tnet.fabricmc.fabric.mixin.lookup.ServerWorldMixin (fabric-api-lookup-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.rule.keepMobInLazyChunks.ServerWorldMixin (carpet-tis-addition.mixins.json)
\tnet.minecraft.server.MinecraftServer:
\t\tnet.fabricmc.fabric.mixin.registry.sync.MinecraftServerMixin (fabric-registry-sync-v0.mixins.json)
\t\torg.embeddedt.modernfix.fabric.mixin.core.MinecraftServerMixin (modernfix-fabric.mixins.json)
\t\tcarpet.mixins.MinecraftServer_pingPlayerSampleLimit (carpet.mixins.json)
\t\tcarpet.mixins.MinecraftServer_coreMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.rule.syncServerMsptMetricsData.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tch.endte.syncmatica.mixin.MixinMinecraftServer (syncmatica.mixin.json)
\t\tcarpet.mixins.MinecraftServer_scarpetMixin (carpet.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.asynctask.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tru.vidtu.ksyxis.mixins.MinecraftServerMixin (ksyxis.mixins.json)
\t\torg.embeddedt.modernfix.common.mixin.perf.dedicated_reload_executor.MinecraftServerMixin (modernfix-common.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.api.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.biome.modification.MinecraftServerMixin (fabric-biome-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.logger.microtiming.messageflush.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.resource.loader.MinecraftServerMixin (fabric-resource-loader-v0.mixins.json)
\t\tcarpet.mixins.MinecraftServer_tickspeedMixin (carpet.mixins.json)
\t\tnet.fabricmc.fabric.mixin.event.lifecycle.MinecraftServerMixin (fabric-lifecycle-events-v1.mixins.json)
\t\tfi.dy.masa.servux.mixin.MixinMinecraftServer (mixins.servux.json)
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.autosave.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tnet.fabricmc.fabric.mixin.message.MinecraftServerMixin (fabric-message-api-v1.mixins.json)
\t\tcarpettisaddition.mixins.carpet.hooks.onServerLoadedWorlds.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcarpettisaddition.mixins.rule.yeetUpdateSuppressionCrash.yeet.MinecraftServerMixin (carpet-tis-addition.mixins.json)
\t\tcom.sk89q.worldedit.fabric.mixin.MixinMinecraftServer (worldedit-fabric.mixins.json)
\tnet.minecraft.class_3176:
\t\tcarpettisaddition.mixins.logger.microtiming.tickstages.console.MinecraftDedicatedServerMixin (carpet-tis-addition.mixins.json)
\t\tdev.architectury.mixin.fabric.MixinDedicatedServer (architectury.mixins.json)

-- System Details --
Details:
\tMinecraft Version: 1.20.4
\tMinecraft Version ID: 1.20.4
\tOperating System: Linux (aarch64) version 5.15.0-1017-oracle
\tJava Version: 17.0.4, Private Build
\tJava VM Version: OpenJDK 64-Bit Server VM (mixed mode, sharing), Private Build
\tMemory: 2160093184 bytes (2060 MiB) / 6442450944 bytes (6144 MiB) up to 6442450944 bytes (6144 MiB)
\tCPUs: 4
\tProcessor Vendor: ARM
\tProcessor Name: 
\tIdentifier: ARM Family 8 Model 0xd0c Stepping r3p1
\tMicroarchitecture: unknown
\tFrequency (GHz): -0.00
\tNumber of physical packages: 1
\tNumber of physical CPUs: 4
\tNumber of logical CPUs: 4
\tGraphics card #0 name: Virtio GPU
\tGraphics card #0 vendor: Red Hat, Inc.
\tGraphics card #0 VRAM (MB): 0.00
\tGraphics card #0 deviceId: unknown
\tGraphics card #0 versionInfo: version: 01
\tVirtual memory max (MB): 11994.09
\tVirtual memory used (MB): 17033.96
\tSwap memory total (MB): 0.00
\tSwap memory used (MB): 0.00
\tJVM Flags: 2 total; -Xmx6G -Xms6G
\tLoaded Scarpet Apps: 
\t\tplayerme
\tFabric Mods: 
\t\tarchitectury: Architectury 11.1.13
\t\tcarpet: Carpet Mod 1.4.128+v231205
\t\tcarpet-extra: Carpet Extra 1.4.128
\t\tcarpet-tis-addition: Carpet TIS Addition 1.62.0
\t\t\tconditional-mixin: conditional mixin 0.6.2
\t\t\tmixinextras: MixinExtras 0.3.6
\t\tfabric-api: Fabric API 0.96.4+1.20.4
\t\t\tfabric-api-base: Fabric API Base 0.4.36+78d798af4f
\t\t\tfabric-api-lookup-api-v1: Fabric API Lookup API (v1) 1.6.49+82b1bb3e4f
\t\t\tfabric-biome-api-v1: Fabric Biome API (v1) 13.0.16+78d798af4f
\t\t\tfabric-block-api-v1: Fabric Block API (v1) 1.0.16+3e2216cb4f
\t\t\tfabric-block-view-api-v2: Fabric BlockView API (v2) 1.0.4+78d798af4f
\t\t\tfabric-command-api-v1: Fabric Command API (v1) 1.2.41+f71b366f4f
\t\t\tfabric-command-api-v2: Fabric Command API (v2) 2.2.20+78d798af4f
\t\t\tfabric-commands-v0: Fabric Commands (v0) 0.2.58+df3654b34f
\t\t\tfabric-containers-v0: Fabric Containers (v0) 0.1.86+df3654b34f
\t\t\tfabric-content-registries-v0: Fabric Content Registries (v0) 5.0.14+78d798af4f
\t\t\tfabric-convention-tags-v1: Fabric Convention Tags 1.5.10+78d798af4f
\t\t\tfabric-crash-report-info-v1: Fabric Crash Report Info (v1) 0.2.23+78d798af4f
\t\t\tfabric-data-attachment-api-v1: Fabric Data Attachment API (v1) 1.1.3+b90db5744f
\t\t\tfabric-data-generation-api-v1: Fabric Data Generation API (v1) 13.2.3+5c0133444f
\t\t\tfabric-dimensions-v1: Fabric Dimensions API (v1) 2.1.61+78d798af4f
\t\t\tfabric-entity-events-v1: Fabric Entity Events (v1) 1.6.1+09fc25014f
\t\t\tfabric-events-interaction-v0: Fabric Events Interaction (v0) 0.7.1+389931eb4f
\t\t\tfabric-events-lifecycle-v0: Fabric Events Lifecycle (v0) 0.2.74+df3654b34f
\t\t\tfabric-game-rule-api-v1: Fabric Game Rule API (v1) 1.0.46+78d798af4f
\t\t\tfabric-item-api-v1: Fabric Item API (v1) 2.2.0+d6f2b0844f
\t\t\tfabric-item-group-api-v1: Fabric Item Group API (v1) 4.0.25+58f8c0124f
\t\t\tfabric-lifecycle-events-v1: Fabric Lifecycle Events (v1) 2.3.0+a67ffb5d4f
\t\t\tfabric-loot-api-v2: Fabric Loot API (v2) 2.1.8+78d798af4f
\t\t\tfabric-message-api-v1: Fabric Message API (v1) 6.0.5+78d798af4f
\t\t\tfabric-mining-level-api-v1: Fabric Mining Level API (v1) 2.1.64+78d798af4f
\t\t\tfabric-networking-api-v1: Fabric Networking API (v1) 3.1.7+2e5ac5484f
\t\t\tfabric-object-builder-api-v1: Fabric Object Builder API (v1) 13.0.13+080016e44f
\t\t\tfabric-particles-v1: Fabric Particles (v1) 1.1.7+78d798af4f
\t\t\tfabric-recipe-api-v1: Fabric Recipe API (v1) 2.0.20+78d798af4f
\t\t\tfabric-registry-sync-v0: Fabric Registry Sync (v0) 4.0.19+58f8c0124f
\t\t\tfabric-rendering-data-attachment-v1: Fabric Rendering Data Attachment (v1) 0.3.42+73761d2e4f
\t\t\tfabric-rendering-fluids-v1: Fabric Rendering Fluids (v1) 3.1.1+e761c6694f
\t\t\tfabric-resource-conditions-api-v1: Fabric Resource Conditions API (v1) 2.3.14+78d798af4f
\t\t\tfabric-resource-loader-v0: Fabric Resource Loader (v0) 0.11.19+58f8c0124f
\t\t\tfabric-screen-handler-api-v1: Fabric Screen Handler API (v1) 1.3.55+78d798af4f
\t\t\tfabric-transfer-api-v1: Fabric Transfer API (v1) 4.0.11+e84342304f
\t\t\tfabric-transitive-access-wideners-v1: Fabric Transitive Access Wideners (v1) 5.0.14+78d798af4f
\t\tfabricloader: Fabric Loader 0.15.7
\t\tferritecore: FerriteCore 6.0.3
\t\tftbbackups2: FTB Backups 2 1.0.27-Release
\t\tinventorysorter: Inventory Sorter 1.9.0-1.20.4
\t\t\tkyrptconfig: Kyrpt Config 1.5.8-1.20.4
\t\tjava: OpenJDK 64-Bit Server VM 17
\t\tkrypton: Krypton 0.2.6
\t\t\tcom_velocitypowered_velocity-native: velocity-native 3.2.0-SNAPSHOT
\t\tksyxis: Ksyxis 1.2.2
\t\tlazydfu: LazyDFU 0.1.3
\t\tlithium: Lithium 0.12.1
\t\tminecraft: Minecraft 1.20.4
\t\tmixintrace: MixinTrace 1.1.1+1.17
\t\tmodernfix: ModernFix 5.13.0+mc1.20.4
\t\tpolylib: PolyLib 2004.0.3-build.120
\t\t\tteam_reborn_energy: Energy 2.2.0
\t\tservux: Servux 0.1.0
\t\tspark: spark 1.10.58
\t\t\tfabric-permissions-api-v0: fabric-permissions-api 0.2-SNAPSHOT
\t\tsyncmatica: Syncmatica 1.20.4-0.3.11
\t\tthreadtweak: ThreadTweak 1.20.4-0.1.2
\t\ttrade_cycling: Trade Cycling 1.20.4-1.0.11
\t\tvillagerleads: VillagerLeads 0.0.1
\t\tvmp: Very Many Players 0.2.0+beta.7.139
\t\t\tcom_ibm_async_asyncutil: asyncutil 0.1.0
\t\tworldedit: WorldEdit 7.3.0+6678-55745ad
\tServer Running: true
\tPlayer Count: 4 / 20; [class_3222['Shroototem'/2, l='ServerLevel[world]', x=52.10, y=137.28, z=471.58], class_3222['dcman58'/655, l='ServerLevel[world]', x=184.06, y=163.53, z=1408.97], class_3222['Vandvia'/1105, l='ServerLevel[world]', x=-44.03, y=166.51, z=-45.17], EntityPlayerMPFake['redstonefarm'/3660, l='ServerLevel[world]', x=1577.39, y=187.00, z=12469.85]]
\tData Packs: vanilla, fabric, fabric-convention-tags-v1, file/dragon drops v1.3.6 (MC 1.20-1.20.4).zip, file/player head drops v1.1.6 (MC 1.20-1.20.4).zip
\tEnabled Feature Flags: minecraft:vanilla
\tWorld Generation: Stable
\tIs Modded: Definitely; Server brand changed to 'fabric'
\tType: Dedicated Server (map_server.txt)
`;