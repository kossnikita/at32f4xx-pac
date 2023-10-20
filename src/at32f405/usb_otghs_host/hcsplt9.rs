#[doc = "Register `HCSPLT9` reader"]
pub type R = crate::R<HCSPLT9_SPEC>;
#[doc = "Register `HCSPLT9` writer"]
pub type W = crate::W<HCSPLT9_SPEC>;
#[doc = "Field `PRTADDR` reader - Port Address"]
pub type PRTADDR_R = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port Address"]
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub Address"]
pub type HUBADDR_R = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub Address"]
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - Transaction Position"]
pub type XACTPOS_R = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - Transaction Position"]
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPSPLT` reader - Do Complete Split"]
pub type COMPSPLT_R = crate::BitReader;
#[doc = "Field `COMPSPLT` writer - Do Complete Split"]
pub type COMPSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLTENA` reader - Split Enable"]
pub type SPLTENA_R = crate::BitReader;
#[doc = "Field `SPLTENA` writer - Split Enable"]
pub type SPLTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&self) -> COMPSPLT_R {
        COMPSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&self) -> SPLTENA_R {
        SPLTENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT9")
            .field("prtaddr", &format_args!("{}", self.prtaddr().bits()))
            .field("hubaddr", &format_args!("{}", self.hubaddr().bits()))
            .field("xactpos", &format_args!("{}", self.xactpos().bits()))
            .field("compsplt", &format_args!("{}", self.compsplt().bit()))
            .field("spltena", &format_args!("{}", self.spltena().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCSPLT9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<HCSPLT9_SPEC> {
        PRTADDR_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HUBADDR_W<HCSPLT9_SPEC> {
        HUBADDR_W::new(self, 7)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XACTPOS_W<HCSPLT9_SPEC> {
        XACTPOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    #[must_use]
    pub fn compsplt(&mut self) -> COMPSPLT_W<HCSPLT9_SPEC> {
        COMPSPLT_W::new(self, 16)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spltena(&mut self) -> SPLTENA_W<HCSPLT9_SPEC> {
        SPLTENA_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Channel 9 Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCSPLT9_SPEC;
impl crate::RegisterSpec for HCSPLT9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsplt9::R`](R) reader structure"]
impl crate::Readable for HCSPLT9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcsplt9::W`](W) writer structure"]
impl crate::Writable for HCSPLT9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCSPLT9 to value 0"]
impl crate::Resettable for HCSPLT9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
