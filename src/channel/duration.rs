use crate::LarkError;

pub(super) fn infer_audio_duration_ms(data: &[u8]) -> Result<i64, LarkError> {
    parse_opus_ogg_duration_ms(data)
}

pub(super) fn infer_video_duration_ms(data: &[u8]) -> Result<i64, LarkError> {
    parse_mp4_duration_ms(data)
}

fn parse_opus_ogg_duration_ms(data: &[u8]) -> Result<i64, LarkError> {
    if data.len() < 27 {
        return Err(LarkError::IllegalParam(
            "audio duration could not be determined: file too small".into(),
        ));
    }

    let scan_start = data.len().saturating_sub(65_536);
    for idx in (scan_start..=data.len() - 27).rev() {
        if &data[idx..idx + 4] == b"OggS" {
            let mut granule = [0u8; 8];
            granule.copy_from_slice(&data[idx + 6..idx + 14]);
            let granule = i64::from_le_bytes(granule);
            if granule < 0 {
                return Err(LarkError::IllegalParam(
                    "audio duration could not be determined: invalid granule".into(),
                ));
            }
            return Ok(((granule as f64) / 48.0).round() as i64);
        }
    }

    Err(LarkError::IllegalParam(
        "audio duration could not be determined: OggS not found".into(),
    ))
}

fn parse_mp4_duration_ms(data: &[u8]) -> Result<i64, LarkError> {
    let (moov_start, moov_end) =
        find_box_payload(data, 0, data.len(), b"moov").ok_or_else(|| {
            LarkError::IllegalParam("video duration could not be determined: moov not found".into())
        })?;
    let (mvhd_start, _) =
        find_box_payload(data, moov_start, moov_end, b"mvhd").ok_or_else(|| {
            LarkError::IllegalParam("video duration could not be determined: mvhd not found".into())
        })?;
    if mvhd_start + 4 > data.len() {
        return Err(LarkError::IllegalParam(
            "video duration could not be determined: invalid mvhd".into(),
        ));
    }
    let version = data[mvhd_start];
    let (timescale, duration) = if version == 1 {
        if mvhd_start + 32 > data.len() {
            return Err(LarkError::IllegalParam(
                "video duration could not be determined: truncated mvhd".into(),
            ));
        }
        (
            read_u32(data, mvhd_start + 20),
            read_u64(data, mvhd_start + 24) as f64,
        )
    } else {
        if mvhd_start + 20 > data.len() {
            return Err(LarkError::IllegalParam(
                "video duration could not be determined: truncated mvhd".into(),
            ));
        }
        (
            read_u32(data, mvhd_start + 12),
            read_u32(data, mvhd_start + 16) as f64,
        )
    };
    if timescale == 0 {
        return Err(LarkError::IllegalParam(
            "video duration could not be determined: invalid timescale".into(),
        ));
    }
    Ok(((duration / timescale as f64) * 1000.0) as i64)
}

fn find_box_payload(
    data: &[u8],
    mut cursor: usize,
    end: usize,
    target: &[u8; 4],
) -> Option<(usize, usize)> {
    while cursor.checked_add(8)? <= end && cursor.checked_add(8)? <= data.len() {
        let size32 = read_u32(data, cursor) as usize;
        let name = data.get(cursor + 4..cursor + 8)?;
        let (header_len, box_size) = if size32 == 1 {
            if cursor.checked_add(16)? > end || cursor.checked_add(16)? > data.len() {
                return None;
            }
            (16usize, read_u64(data, cursor + 8) as usize)
        } else if size32 == 0 {
            (8usize, end.saturating_sub(cursor))
        } else {
            (8usize, size32)
        };
        if box_size < header_len {
            return None;
        }
        let box_end = cursor.checked_add(box_size)?;
        if box_end > end || box_end > data.len() {
            return None;
        }
        if name == target {
            return Some((cursor + header_len, box_end));
        }
        cursor = box_end;
    }
    None
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..offset + 4]);
    u32::from_be_bytes(bytes)
}

fn read_u64(data: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[offset..offset + 8]);
    u64::from_be_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_opus_ogg_duration_from_final_page() {
        let mut data = vec![0u8; 64];
        data[10..14].copy_from_slice(b"OggS");
        data[16..24].copy_from_slice(&48_000i64.to_le_bytes());

        assert_eq!(infer_audio_duration_ms(&data).unwrap(), 1000);
    }

    #[test]
    fn infers_mp4_duration_from_mvhd_v0() {
        let mp4 = minimal_mp4(1_000, 5_000);
        assert_eq!(infer_video_duration_ms(&mp4).unwrap(), 5000);
    }

    fn minimal_mp4(timescale: u32, duration: u32) -> Vec<u8> {
        let mut mvhd_payload = Vec::new();
        mvhd_payload.extend_from_slice(&[0, 0, 0, 0]);
        mvhd_payload.extend_from_slice(&0u32.to_be_bytes());
        mvhd_payload.extend_from_slice(&0u32.to_be_bytes());
        mvhd_payload.extend_from_slice(&timescale.to_be_bytes());
        mvhd_payload.extend_from_slice(&duration.to_be_bytes());

        let mut mvhd = Vec::new();
        mvhd.extend_from_slice(&((mvhd_payload.len() + 8) as u32).to_be_bytes());
        mvhd.extend_from_slice(b"mvhd");
        mvhd.extend_from_slice(&mvhd_payload);

        let mut moov = Vec::new();
        moov.extend_from_slice(&((mvhd.len() + 8) as u32).to_be_bytes());
        moov.extend_from_slice(b"moov");
        moov.extend_from_slice(&mvhd);
        moov
    }
}
