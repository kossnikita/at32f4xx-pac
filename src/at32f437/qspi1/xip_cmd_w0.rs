#[doc = "Register `XIP_CMD_W0` reader"]
pub type R = crate::R<XIP_CMD_W0_SPEC>;
#[doc = "Register `XIP_CMD_W0` writer"]
pub type W = crate::W<XIP_CMD_W0_SPEC>;
#[doc = "Field `XIPR_DUM2` reader - XIP read second dummy cycle"]
pub type XIPR_DUM2_R = crate::FieldReader;
#[doc = "Field `XIPR_DUM2` writer - XIP read second dummy cycle"]
pub type XIPR_DUM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XIPR_OPMODE` reader - XIP read operate mode"]
pub type XIPR_OPMODE_R = crate::FieldReader;
#[doc = "Field `XIPR_OPMODE` writer - XIP read operate mode"]
pub type XIPR_OPMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XIPR_ADRLEN` reader - XIP read address length"]
pub type XIPR_ADRLEN_R = crate::BitReader;
#[doc = "Field `XIPR_ADRLEN` writer - XIP read address length"]
pub type XIPR_ADRLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XIPR_INSC` reader - XIP read instruction code"]
pub type XIPR_INSC_R = crate::FieldReader;
#[doc = "Field `XIPR_INSC` writer - XIP read instruction code"]
pub type XIPR_INSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - XIP read second dummy cycle"]
    #[inline(always)]
    pub fn xipr_dum2(&self) -> XIPR_DUM2_R {
        XIPR_DUM2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - XIP read operate mode"]
    #[inline(always)]
    pub fn xipr_opmode(&self) -> XIPR_OPMODE_R {
        XIPR_OPMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - XIP read address length"]
    #[inline(always)]
    pub fn xipr_adrlen(&self) -> XIPR_ADRLEN_R {
        XIPR_ADRLEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - XIP read instruction code"]
    #[inline(always)]
    pub fn xipr_insc(&self) -> XIPR_INSC_R {
        XIPR_INSC_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W0")
            .field("xipr_dum2", &format_args!("{}", self.xipr_dum2().bits()))
            .field(
                "xipr_opmode",
                &format_args!("{}", self.xipr_opmode().bits()),
            )
            .field("xipr_adrlen", &format_args!("{}", self.xipr_adrlen().bit()))
            .field("xipr_insc", &format_args!("{}", self.xipr_insc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<XIP_CMD_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - XIP read second dummy cycle"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_dum2(&mut self) -> XIPR_DUM2_W<XIP_CMD_W0_SPEC, 0> {
        XIPR_DUM2_W::new(self)
    }
    #[doc = "Bits 8:10 - XIP read operate mode"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_opmode(&mut self) -> XIPR_OPMODE_W<XIP_CMD_W0_SPEC, 8> {
        XIPR_OPMODE_W::new(self)
    }
    #[doc = "Bit 11 - XIP read address length"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_adrlen(&mut self) -> XIPR_ADRLEN_W<XIP_CMD_W0_SPEC, 11> {
        XIPR_ADRLEN_W::new(self)
    }
    #[doc = "Bits 12:19 - XIP read instruction code"]
    #[inline(always)]
    #[must_use]
    pub fn xipr_insc(&mut self) -> XIPR_INSC_W<XIP_CMD_W0_SPEC, 12> {
        XIPR_INSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "XIP command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIP_CMD_W0_SPEC;
impl crate::RegisterSpec for XIP_CMD_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w0::R`](R) reader structure"]
impl crate::Readable for XIP_CMD_W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w0::W`](W) writer structure"]
impl crate::Writable for XIP_CMD_W0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W0 to value 0"]
impl crate::Resettable for XIP_CMD_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
