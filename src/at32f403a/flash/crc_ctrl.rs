#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CRC_CTRL_SPEC>;
#[doc = "Field `CRC_SS` writer - CRC start sector"]
pub type CRC_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CRC_SN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CRC_STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CRC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:11 - CRC start sector"]
    #[inline(always)]
    pub fn crc_ss(&mut self) -> CRC_SS_W<'_, CRC_CTRL_SPEC> {
        CRC_SS_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&mut self) -> CRC_SN_W<'_, CRC_CTRL_SPEC> {
        CRC_SN_W::new(self, 12)
    }
    #[doc = "Bit 31 - CRC start"]
    #[inline(always)]
    pub fn crc_strt(&mut self) -> CRC_STRT_W<'_, CRC_CTRL_SPEC> {
        CRC_STRT_W::new(self, 31)
    }
}
#[doc = "CRC controler register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CTRL_SPEC;
impl crate::RegisterSpec for CRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl::W`](W) writer structure"]
impl crate::Writable for CRC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_CTRL to value 0"]
impl crate::Resettable for CRC_CTRL_SPEC {}
