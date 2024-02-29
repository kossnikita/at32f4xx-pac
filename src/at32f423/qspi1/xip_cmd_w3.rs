#[doc = "Register `XIP_CMD_W3` reader"]
pub type R = crate::R<XIP_CMD_W3_SPEC>;
#[doc = "Register `XIP_CMD_W3` writer"]
pub type W = crate::W<XIP_CMD_W3_SPEC>;
#[doc = "Field `BYPASSC` reader - Bypass cache function"]
pub type BYPASSC_R = crate::BitReader;
#[doc = "Field `BYPASSC` writer - Bypass cache function"]
pub type BYPASSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTS` reader - Cache status"]
pub type CSTS_R = crate::BitReader;
#[doc = "Field `CSTS` writer - Cache status"]
pub type CSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bypass cache function"]
    #[inline(always)]
    pub fn bypassc(&self) -> BYPASSC_R {
        BYPASSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Cache status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W3")
            .field("bypassc", &format_args!("{}", self.bypassc().bit()))
            .field("csts", &format_args!("{}", self.csts().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<XIP_CMD_W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass cache function"]
    #[inline(always)]
    #[must_use]
    pub fn bypassc(&mut self) -> BYPASSC_W<XIP_CMD_W3_SPEC> {
        BYPASSC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Cache status"]
    #[inline(always)]
    #[must_use]
    pub fn csts(&mut self) -> CSTS_W<XIP_CMD_W3_SPEC> {
        CSTS_W::new(self, 3)
    }
}
#[doc = "XIP command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIP_CMD_W3_SPEC;
impl crate::RegisterSpec for XIP_CMD_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w3::R`](R) reader structure"]
impl crate::Readable for XIP_CMD_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w3::W`](W) writer structure"]
impl crate::Writable for XIP_CMD_W3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W3 to value 0"]
impl crate::Resettable for XIP_CMD_W3_SPEC {
    const RESET_VALUE: u32 = 0;
}
