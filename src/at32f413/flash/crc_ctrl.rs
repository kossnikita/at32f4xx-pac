#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CRC_CTRL_SPEC>;
#[doc = "Field `CRC_SS` writer - CRC start sector"]
pub type CRC_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CRC_SN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CRC_STRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<CRC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:6 - CRC start sector"]
    #[inline(always)]
    #[must_use]
    pub fn crc_ss(&mut self) -> CRC_SS_W<CRC_CTRL_SPEC, 0> {
        CRC_SS_W::new(self)
    }
    #[doc = "Bits 7:13 - CRC sector numbler"]
    #[inline(always)]
    #[must_use]
    pub fn crc_sn(&mut self) -> CRC_SN_W<CRC_CTRL_SPEC, 7> {
        CRC_SN_W::new(self)
    }
    #[doc = "Bit 14 - CRC start"]
    #[inline(always)]
    #[must_use]
    pub fn crc_strt(&mut self) -> CRC_STRT_W<CRC_CTRL_SPEC, 14> {
        CRC_STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash CRC controler register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CTRL_SPEC;
impl crate::RegisterSpec for CRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl::W`](W) writer structure"]
impl crate::Writable for CRC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_CTRL to value 0"]
impl crate::Resettable for CRC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
