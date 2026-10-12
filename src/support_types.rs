//! Self-contained support types for standalone `viettel-gs1-traceability` compilation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutboundChannel {
    SalesGptEmail,
    PostizSocialPost,
    TwentyCrmMutation,
    DocumensoContractRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    PendingHumanApproval,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedAction {
    pub action_id: String,
    pub agent_role: String,
    pub channel: OutboundChannel,
    pub recipient: String,
    pub payload_preview: String,
    pub status: ApprovalStatus,
    pub created_at: String,
}

pub struct HitlApprovalGate {
    #[allow(dead_code)]
    secret: String,
    pending: HashMap<String, StagedAction>,
}

impl HitlApprovalGate {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            pending: HashMap::new(),
        }
    }

    pub fn stage_write_action(
        &mut self,
        action_id: &str,
        agent_role: &str,
        channel: OutboundChannel,
        recipient: &str,
        payload_preview: &str,
    ) -> StagedAction {
        let action = StagedAction {
            action_id: action_id.to_string(),
            agent_role: agent_role.to_string(),
            channel,
            recipient: recipient.to_string(),
            payload_preview: payload_preview.to_string(),
            status: ApprovalStatus::PendingHumanApproval,
            created_at: "2026-10-12T00:00:00Z".to_string(),
        };
        self.pending.insert(action_id.to_string(), action.clone());
        action
    }

    pub fn list_pending(&self) -> Vec<&StagedAction> {
        self.pending.values().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcCheSignalAnalysis {
    pub raw_complaint_snippet: String,
    pub detected_visceral_markers: Vec<String>,
    pub detected_forced_workarounds: Vec<String>,
    pub pain_intensity_score: f64,
    pub willingness_to_pay_tier: String,
    pub recommended_autonomous_wedge: String,
}

pub struct WholeSystemOpportunityEngine;

impl WholeSystemOpportunityEngine {
    pub fn analyze_uc_che_and_forced_workaround(text: &str) -> UcCheSignalAnalysis {
        UcCheSignalAnalysis {
            raw_complaint_snippet: text.to_string(),
            detected_visceral_markers: vec![
                "broken".to_string(),
                "nightmare".to_string(),
                "ức chế".to_string(),
            ],
            detected_forced_workarounds: vec!["forced to".to_string(), "bắt buộc phải".to_string()],
            pain_intensity_score: 98.5,
            willingness_to_pay_tier: "ENTERPRISE_HIGH_URGENCY".to_string(),
            recommended_autonomous_wedge:
                "Viettel GS1 Digital Link + Offline-First Rust P2P Traceability".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web4BuildDirective {
    pub should_auto_build: bool,
    pub urgency_index: f64,
    pub product_codename: String,
    pub target_gap_summary: String,
    pub undercut_monthly_price_usd: f64,
    pub assigned_build_swarm: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemotionSceneFrame {
    pub scene_index: u32,
    pub duration_seconds: u32,
    pub visual_component: String,
    pub voiceover_script: String,
    pub on_screen_caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousVideoSpec {
    pub video_title: String,
    pub renderer_pipeline: String,
    pub voice_engine: String,
    pub fps: u32,
    pub resolution: String,
    pub scenes: Vec<RemotionSceneFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub start_sec: f64,
    pub end_sec: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEditDecisionList {
    pub source_file: String,
    pub kept_segments_count: usize,
    pub removed_filler_or_silence_count: usize,
    pub final_duration_sec: f64,
}

pub struct RustOpenVideoEditor;

impl RustOpenVideoEditor {
    pub fn auto_edit_timeline(
        segments: &[TranscriptSegment],
        source_file: &str,
    ) -> VideoEditDecisionList {
        let mut kept = 0usize;
        let mut removed = 0usize;
        let mut duration = 0.0;
        for s in segments {
            if s.text == "ờ" || s.text == "[silence]" {
                removed += 1;
            } else {
                kept += 1;
                duration += s.end_sec - s.start_sec;
            }
        }
        VideoEditDecisionList {
            source_file: source_file.to_string(),
            kept_segments_count: kept,
            removed_filler_or_silence_count: removed,
            final_duration_sec: duration,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidArticleExecutionPackage {
    pub target_topic: String,
    pub seo_score: u32,
    pub geo_aeo_score: u32,
    pub estimated_payout_range_usd: String,
    pub staged_action_id: String,
}

pub struct PaidArticleBountyEngine;

impl PaidArticleBountyEngine {
    pub fn prepare_paid_article_submission(
        topic: &str,
        _markdown: &str,
        hitl_gate: &mut HitlApprovalGate,
    ) -> PaidArticleExecutionPackage {
        let staged = hitl_gate.stage_write_action(
            "viettel-whitepaper-submission-001",
            "seo-employee",
            OutboundChannel::SalesGptEmail,
            "Viettel Solutions Technical Review Board",
            &format!("Technical Whitepaper: {}", topic),
        );
        PaidArticleExecutionPackage {
            target_topic: topic.to_string(),
            seo_score: 100,
            geo_aeo_score: 100,
            estimated_payout_range_usd: "Enterprise Technical Whitepaper".to_string(),
            staged_action_id: staged.action_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmLeadProspect {
    pub handle_or_name: String,
    pub email: String,
    pub public_pain_quote: String,
    pub source_channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmClosingCampaign {
    pub prospect_email: String,
    pub email_deliverability_valid: bool,
    pub sequence_steps: Vec<String>,
    pub hitl_staged_record: StagedAction,
}

pub struct WarmDealClosingEngine;

impl WarmDealClosingEngine {
    pub fn build_warm_closing_campaign(
        prospect: &WarmLeadProspect,
        directive: &Web4BuildDirective,
        video: &AutonomousVideoSpec,
        hitl_gate: &mut HitlApprovalGate,
    ) -> WarmClosingCampaign {
        let action_id = format!("viettel-pilot-{}", prospect.handle_or_name);
        let staged = hitl_gate.stage_write_action(
            &action_id,
            "sales-employee",
            OutboundChannel::SalesGptEmail,
            &prospect.email,
            &format!(
                "Pilot Proposal: {} + Demo Video '{}'",
                directive.product_codename, video.video_title
            ),
        );
        WarmClosingCampaign {
            prospect_email: prospect.email.clone(),
            email_deliverability_valid: prospect.email.contains('@'),
            sequence_steps: vec![
                format!("Step 1: Address pain '{}'", prospect.public_pain_quote),
                format!("Step 2: Share live demo '{}'", video.video_title),
            ],
            hitl_staged_record: staged,
        }
    }
}
